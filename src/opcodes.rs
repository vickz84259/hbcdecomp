use std::convert::TryFrom;

#[derive(Debug)]
#[repr(u8)]
pub enum Opcode {
    /// Create an object from a static map of values, as for var={'a': 3}.
    /// Any non-constant elements can be set afterwards with PutOwnByInd.
    /// Arg1 is the destination.
    /// Arg2 is a preallocation size hint.
    /// Arg3 is the number of static elements.
    /// Arg4 is the index in the object key buffer table.
    /// Arg5 is the index in the object val buffer table.
    NewObjectWithBuffer = 0,
    NewObjectWithBufferLong = 1,

    /// Create a new, empty Object using the built-in constructor (regardless of
    /// whether it was overridden).
    /// Arg1 = {}
    NewObject = 2,

    /// Create a new empty Object with the specified parent. If the parent is
    /// null, no parent is used. If the parent is not an object, the builtin
    /// Object.prototype is used. Otherwise the parent itself is used.
    /// Arg1 = the created object
    /// Arg2 = the parent.
    NewObjectWithParent = 3,

    /// Create an array from a static list of values, as for var=[1,2,3].
    /// Any non-constant elements can be set afterwards with PutOwnByIndex.
    /// Arg1 is the destination.
    /// Arg2 is a preallocation size hint.
    /// Arg3 is the number of static elements.
    /// Arg4 is the index in the array buffer table.
    NewArrayWithBuffer = 4,
    NewArrayWithBufferLong = 5,

    /// Create a new array of a given size.
    /// Arg1 = new Array(Arg2)
    NewArray = 6,

    /// Arg1 = Arg2 (Register copy)
    Mov = 7,

    /// Arg1 = Arg2 (Register copy, long index)
    MovLong = 8,

    /// Arg1 = -Arg2 (Unary minus)
    Negate = 9,

    /// Arg1 = !Arg2 (Boolean not)
    Not = 10,

    /// Arg1 = ~Arg2 (Bitwise not)
    BitNot = 11,

    /// Arg1 = typeof Arg2 (JS typeof)
    TypeOf = 12,

    /// Arg1 = Arg2 == Arg3 (JS equality)
    Eq = 13,

    /// Arg1 = Arg2 === Arg3 (JS strict equality)
    StrictEq = 14,

    /// Arg1 = Arg2 != Arg3 (JS inequality)
    Neq = 15,

    /// Arg1 = Arg2 !== Arg3 (JS strict inequality)
    StrictNeq = 16,

    /// Arg1 = Arg2 < Arg3 (JS less-than)
    Less = 17,

    /// Arg1 = Arg2 <= Arg3 (JS less-than-or-equals)
    LessEq = 18,

    /// Arg1 = Arg2 > Arg3 (JS greater-than)
    Greater = 19,

    /// Arg1 = Arg2 >= Arg3 (JS greater-than-or-equals)
    GreaterEq = 20,

    /// Arg1 = Arg2 + Arg3 (JS addition/concatenation)
    Add = 21,

    /// Arg1 = Arg2 + Arg3 (Numeric addition, skips number check)
    AddN = 22,

    /// Arg1 = Arg2 * Arg3 (JS multiplication)
    Mul = 23,

    /// Arg1 = Arg2 * Arg3 (Numeric multiplication, skips number check)
    MulN = 24,

    /// Arg1 = Arg2 / Arg3 (JS division)
    Div = 25,

    /// Arg1 = Arg2 / Arg3 (Numeric division, skips number check)
    DivN = 26,

    /// Arg1 = Arg2 % Arg3 (JS remainder)
    Mod = 27,

    /// Arg1 = Arg2 - Arg3 (JS subtraction)
    Sub = 28,

    /// Arg1 = Arg2 - Arg3 (Numeric subtraction, skips number check)
    SubN = 29,

    /// Arg1 = Arg2 << Arg3 (JS bitshift left)
    LShift = 30,

    /// Arg1 = Arg2 >> Arg3 (JS signed bitshift right)
    Rshift = 31,

    /// Arg1 = Arg2 >>> Arg3 (JS unsigned bitshift right)
    URShift = 32,

    /// Arg1 = Arg2 & Arg3 (JS bitwise AND)
    BitAnd = 33,

    /// Arg1 = Arg2 ^ Arg3 (JS bitwise XOR)
    BitXor = 34,

    /// Arg1 = Arg2 | Arg3 (JS bitwise OR)
    BitOr = 35,

    /// Check whether Arg2 contains Arg3 in its prototype chain.
    /// Note that this is not the same as JS instanceof.
    /// Pseudocode: Arg1 = prototypechain(Arg2).contains(Arg3)
    InstanceOf = 36,

    /// Arg1 = Arg2 in Arg3 (JS relational 'in')
    IsIn = 37,

    /// Get an environment (scope) from N levels up the stack.
    /// 0 is the current environment, 1 is the caller's environment, etc.
    GetEnvironment = 38,

    /// Store a value in an environment.
    /// StoreNPToEnvironment[L] store a non-pointer value in an environment
    /// Arg1 is the environment (as fetched by GetEnvironment).
    /// Arg2 is the environment index slot number.
    /// Arg3 is the value.
    StoreToEnvironment = 39,
    StoreToEnvironmentL = 40,
    StoreNPToEnvironment = 41,
    StoreNPToEnvironmentL = 42,

    /// Load a value from an environment.
    /// Arg1 is the destination.
    /// Arg2 is the environment (as fetched by GetEnvironment).
    /// Arg3 is the environment index slot number.
    LoadFromEnvironment = 43,
    LoadFromEnvironmentL = 44,

    /// Get the global object (the object in which global variables are stored).
    GetGlobalObject = 45,

    /// Obtain the value of NewTarget from the frame.
    /// Arg1 = NewTarget
    GetNewTarget = 46,

    /// Create a new environment, to store values captured by closures.
    CreateEnvironment = 47,

    /// Declare a global variable by string table index.
    /// The variable will be set to undefined.
    DeclareGlobalVar = 48,

    /// Get an object property by string table index.
    /// Arg1 = Arg2[stringtable[Arg4]]
    /// Arg3 is a cache index used to speed up the above operation.
    GetByIdShort = 49,
    GetById = 50,
    GetByIdLong = 51,

    /// Get an object property by string table index, or throw if not found.
    /// This is similar to GetById, but intended for use with global variables
    /// where Arg2 = GetGlobalObject.
    TryGetById = 52,
    TryGetByIdLong = 53,

    /// Set an object property by string index.
    /// Arg1[stringtable[Arg4]] = Arg2.
    PutById = 54,
    PutByIdLong = 55,

    /// Set an object property by string index, or throw if undeclared.
    /// This is similar to PutById, but intended for use with global variables
    /// where Arg1 = GetGlobalObject.
    TryPutById = 56,
    TryPutByIdLong = 57,

    /// Create a new own property on an object. This is similar to PutById, but
    /// the destination must be an object, it only deals with own properties,
    /// ignoring the prototype chain, and the property must not already be defined.
    /// Similarly to PutById, the property name cannot be a valid array index.
    /// Arg1 is the destination object, which is known to be an object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    PutNewOwnByIdShort = 58,
    PutNewOwnById = 59,
    PutNewOwnByIdLong = 60,

    /// Create a new non-enumerable own property on an object. This is the same as
    /// PutNewOwnById, but creates the property with different enumerability.
    /// Arg1 is the destination object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    PutNewOwnNEById = 61,
    PutNewOwnNEByIdLong = 62,

    /// Assign a value to a constant integer own property which will be created as
    /// enumerable. This is used (potentially in conjunction with
    /// NewArrayWithBuffer) for arr=[foo,bar] initializations.
    /// Arg1[Arg3] = Arg2;
    PutOwnByIndex = 63,
    PutOwnByIndexL = 64,

    /// Set an own property identified by value.
    /// Arg1 is the destination object.
    /// Arg2 is the value to write.
    /// Arg3 is the property name.
    /// Arg4 : bool -> enumerable. If true, the property is created as enumerable,
    ///        non-enumerable otherwise.
    /// Arg1[Arg3] = Arg2;
    PutOwnByVal = 65,

    /// Delete a property by string table index.
    /// Arg1 = delete Arg2[stringtable[Arg3]]
    DelById = 66,
    DelByIdLong = 67,

    /// Get a property by value. Constants string values should instead use GetById.
    /// Arg1 = Arg2[Arg3]
    GetByVal = 68,

    /// Set a property by value. Constant string values should instead use GetById
    /// (unless they are array indices according to ES5.1 section 15.4, in which
    /// case this is still the right opcode).
    /// Arg1[Arg2] = Arg3
    PutByVal = 69,

    /// Delete a property by value (when the value is not known at compile time).
    /// Arg1 = delete Arg2[Arg3]
    DelByVal = 70,

    /// Add a getter and a setter for a property by value.
    /// Object.defineProperty(Arg1, Arg2, { get: Arg3, set: Arg4 }).
    /// Arg1 is the target object which will have a property defined.
    /// Arg2 is the property name
    /// Arg3 is the getter closure or undefined
    /// Arg4 is the setter closure or undefined
    /// Arg5 : boolean - if true, the property will be enumerable.
    PutOwnGetterSetterByVal = 71,

    /// Get the list of properties from an object to implement for..in loop.
    /// Returns Arg1, which is the register that holds array of properties.
    /// Returns Undefined if the object is null/undefined.
    /// Arg2 is the register that holds the object.
    /// Arg3 is the register that holds the iterating index.
    /// Arg4 is the register that holds the size of the property list.
    GetPNameList = 72,

    /// Get the next property in the for..in iterator.
    /// Returns Arg1, which is the next property. Undefined if unavailable.
    /// Arg2 is the register that holds array of properties.
    /// Arg3 is the register that holds the object.
    /// Arg4 is the register that holds the iterating index.
    /// Arg5 is the register that holds the size of the property list.
    GetNextPName = 73,

    /// Call a function.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///      from the end of the current frame.
    Call = 74,

    /// Call a constructor, with semantics identical to Call.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///      from the end of the current frame. The first argument 'this'
    ///      is assumed to be created with CreateThis.
    Construct = 75,

    /// Call a function with one arg.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    Call1 = 76,

    /// Call a function directly without a closure.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the number of arguments, assumed to be found in reverse order
    ///      from the end of the current frame. The first argument 'this'
    ///      is assumed to be created with CreateThis.
    /// Arg3 is index in the function table.
    /// Note that we expect the variable-sized argument to be last.
    CallDirect = 77,

    /// Call a function with two args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    Call2 = 78,

    /// Call a function with three args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    /// Arg5 is the third argument.
    Call3 = 79,

    /// Call a function with four args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    /// Arg5 is the third argument.
    /// Arg6 is the fourth argument.
    Call4 = 80,

    /// Identical to Call, but allowing more arguments.
    CallLong = 81,

    /// Identical to Construct, but allowing more arguments.
    ConstructLong = 82,

    /// Identical to CallDirect, but the function index is 32-bit.
    CallDirectLongIndex = 83,

    /// Call a builtin function.
    /// Note this is NOT marked as a Ret target, because the callee is native
    /// and therefore never JS.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the builtin number.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///      from the end of the current frame.
    /// thisArg is set to "undefined".
    CallBuiltin = 84,

    /// Return a value from the current function.
    /// return Arg1;
    Ret = 85,

    /// Catch an exception (the first instruction in an exception handler).
    /// } catch(Arg1) {
    Catch = 86,

    /// ES6 18.2.1.1 PerformEval(Arg2, evalRealm, strictCaller=true, direct=true)
    /// Arg1 is the destination of the return value.
    /// Arg2 is the value to eval.
    DirectEval = 87,

    /// Throw an exception.
    /// throw Arg1;
    Throw = 88,

    /// Throw ReferenceError if the operand is HermesValue::undefined
    ThrowIfUndefindedInst = 89,

    /// Implementation dependent debugger action.
    Debugger = 90,

    /// Fast check for an async interrupt request.
    AsyncBreakCheck = 91,

    /// Define a profile point.
    /// Arg1 is the function local profile point index. The first one will have the
    /// largest index. If there are more than 2^16 profile points in the function,
    /// all the overflowed profile points have index zero.
    ProfilePoint = 92,

    /// Unreachable opcode for stubs and similar.
    Unreachable = 93,

    /// Create a closure.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    CreateClosure = 94,
    CreateClosureLongIndex = 95,

    /// Create a closure for a GeneratorFunction.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    CreateGeneratorClosure = 96,
    CreateGeneratorClosureLongIndex = 97,

    /// Allocate an empty, uninitialized object (immediately before a constructor).
    /// Arg1 is the destination register.
    /// Arg2 is the prototype to assign it.
    /// Arg3 is the constructor closure that will be used*.
    /// * To allow internal constructors to have special objects allocated.
    CreateThis = 98,

    /// Choose the result of a constructor: 'this' or a returned object.
    /// Arg1 is the result.
    /// Arg2 is the 'this' object used for the constructor.
    /// Arg3 is the constructor's return value.
    /// Arg1 = Arg3 instanceof Object ? Arg3 : Arg2
    SelectObject = 99,

    /// Load a function parameter by index. Starts at 0 with 'this'.
    /// Arg1 = Arg2 == 0 ? this : arguments[Arg2 - 1];
    LoadParam = 100,

    /// Like LoadParam, but allows accessing arguments >= 255.
    LoadParamLong = 101,

    /// Load a constant integer value.
    LoadConstUInt8 = 102,
    LoadConstInt = 103,

    /// Load a constant double value
    LoadConstDouble = 104,

    /// Load a constant string value by string table index.
    LoadConstString = 105,
    LoadConstStringLongIndex = 106,

    /// Load common constants.
    LoadConstUndefined = 107,
    LoadConstNull = 108,
    LoadConstTrue = 109,
    LoadConstFalse = 110,
    LoadConstZero = 111,

    /// Coerce a value assumed to contain 'this' to an object using non-strict
    /// mode rules. Primitives are boxed, \c null or \c undefed produce the global
    /// object.
    /// Arg1 = coerce_to_object(Arg2)
    CoerceThisNS = 112,

    /// Obtain the raw \c this value and coerce it to an object. Equivalent to:
    /// \code
    ///     LoadParam    Arg1, #0
    ///     CoerceThisNS Arg1, Arg1
    /// \endcode
    LoadThisNS = 113,

    /// Convert a value to a number.
    /// Arg1 = Arg2 - 0
    ToNumber = 114,

    /// Convert a value to a 32-bit signed integer.
    /// Arg1 = Arg2 | 0
    ToInt32 = 115,

    /// Convert a value to a string as if evaluating the expression:
    ///     Arg1 = "" + Arg2
    /// In practice this means
    ///     Arg1 = ToString(ToPrimitive(Arg2, PreferredType::NONE))
    /// with ToPrimitive (ES5.1 9.1) and ToString (ES5.1 9.8).
    AddEmptyString = 116,

    // `arguments` opcodes all work with a lazy register that contains either
    // undefined or a reified array. On the first ReifyArguments, the register
    // will be populated and the rest of the instruction will access it directly.
    // This is an optimization to allow arguments[i] to just load an argument
    // instead of doing a full array allocation and property lookup.
    /// Get a property of the 'arguments' array by value.
    /// Arg1 is the result.
    /// Arg2 is the index.
    /// Arg3 is the lazy loaded register.
    /// Arg1 = arguments[Arg2]
    GetArgumentsPropByVal = 117,

    /// Get the length of the 'arguments' array.
    /// Arg1 is the result.
    /// Arg2 is the lazy loaded register.
    /// Arg1 = arguments.length
    GetArgumentsLength = 118,

    /// Create a regular expression.
    /// Arg1 is the result.
    /// Arg2 is the string index of the pattern.
    /// Arg3 is the string index of the flags.
    /// Arg4 is the regexp bytecode index in the regexp table.
    CreateRegExp = 119,

    /// Jump table switch - using a table of offset, jump to the offset of the given
    /// input or to the default block if out of range (or not right type)
    /// Arg 1 is the value to be branched upon
    /// Arg 2 is the relative offset of the jump table to be used by this
    /// instruction. Jump tables are appended to the bytecode. Arg 3 is the relative
    /// offset for the "default" jump. Arg 4 is the unsigned min value, if arg 1 is
    /// less than this value jmp to
    ///   default block
    /// Arg 5 is the unsigned max value, if arg 1 is greater than this value jmp to
    ///   default block.
    ///
    /// Given the above, the jump table entry for a given value (that is in range)
    /// is located at offset ip + arg2 + arg1 - arg4. We subtract arg4 to avoid
    /// wasting space when compiling denses switches that do not start at zero. Note
    /// that Arg2 is *unaligned* it is dynamically aligned at runtime.
    SwitchImm = 120,

    /// Start the generator by jumping to the next instruction to begin.
    /// Restore the stack frame if this generator has previously been suspended.
    StartGenerator = 121,

    /// Resume generator by performing one of the following user-requested actions:
    /// - next(val): Set Arg1 to val, Arg2 to false, run next instruction
    /// - return(val): Set Arg1 to val, Arg2 to true, run next instruction
    /// - throw(val): Throw val as an error
    /// Arg1 is the result provided by the user.
    /// Arg2 is a boolean which is true if the user requested a return().
    ResumeGenerator = 122,

    /// Set the generator status to complete, but do not return.
    CompleteGenerator = 123,

    /// Create a generator.
    /// Arg1 is the register in which to store the generator.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    CreateGenerator = 124,
    CreateGeneratorLongIndex = 125,

    /// Arg1 [out] is the result iterator or index.
    /// Arg2 [in/out] is the source. Output for either the source or next method.
    /// If source is an array with an unmodified [Symbol.iterator], the result is
    /// 0. Else the result is source[Symbol.iterator] and the output is the .next()
    /// method on the iterator.
    /// See IR.md for IteratorBeginInst.
    IteratorBegin = 126,

    /// Arg1 [out] is the result, or undefined if done.
    /// Arg2 [in/out] is the iterator or index.
    /// Arg2 [in] is the source or the next method.
    /// If iterator is undefined, result = undefined.
    /// If iterator is a number:
    ///   If iterator is less than source.length, return source[iterator++]
    ///   Else iterator = undefined and result = undefined
    /// Else:
    ///   n = iterator.next()
    ///   If n.done, iterator = undefined and result = undefined.
    ///   Else result = n.value
    /// See IR.md for IteratorNextInst.
    IteratorNext = 127,

    /// Arg1 [in] is the iterator or array index.
    /// Arg2 is a bool indicating whether to ignore the inner exception.
    /// If the iterator is an object, call iterator.return().
    /// If Arg2 is true, ignore exceptions which are thrown by iterator.return().
    /// See IR.md for IteratorCloseInst.
    IteratorClose = 128,

    /// Unconditional branch to Arg1.
    Jmp = 129,
    JmpLong = 130,

    /// Conditional branches to Arg1 based on Arg2.
    JmpTrue = 131,
    JmpTrueLong = 132,
    JmpFalse = 133,
    JmpFalseLong = 134,

    /// Jump if the value is undefined.
    JmpUndefined = 135,
    JmpUndefinedLong = 136,

    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    SaveGenerator = 137,
    SaveGeneratorLong = 138,

    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.

    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    JLess = 139,
    JLessLong = 140,
    JNotLess = 141,
    JNotLessLong = 142,

    JLessN = 143,
    JLessNLong = 144,
    JNotLessN = 145,
    JNotLessNLong = 146,

    JLessEqual = 147,
    JLessEqualLong = 148,
    JNotLessEqual = 149,
    JNotLessEqualLong = 150,

    JLessEqualN = 151,
    JLessEqualNLong = 152,
    JNotLessEqualN = 153,
    JNotLessEqualNLong = 154,

    JGreater = 155,
    JGreaterLong = 156,
    JNotGreater = 157,
    JNotGreaterLong = 158,

    JGreaterN = 159,
    JGreaterNLong = 160,
    JNotGreaterN = 161,
    JNotGreaterNLong = 162,

    JGreaterEqual = 163,
    JGreaterEqualLong = 164,
    JNotGreaterEqual = 165,
    JNotGreaterEqualLong = 166,

    JGreaterEqualN = 167,
    JGreaterEqualNLong = 168,
    JNotGreaterEqualN = 169,
    JNotGreaterEqualNLong = 170,

    JEqual = 171,
    JEqualLong = 172,
    JNotEqual = 173,
    JNotEqualLong = 174,

    JStrictEqual = 175,
    JStrictEqualLong = 176,
    JStrictNotEqual = 177,
    JStrictNotEqualLong = 178,
}

impl TryFrom<u8> for Opcode {
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0..=178 => Ok(unsafe { std::mem::transmute(byte) }),
            _ => Err(format!("Unknown Bytecode: {}", byte)),
        }
    }
}

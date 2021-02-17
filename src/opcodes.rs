use std::convert::TryFrom;

#[derive(Debug)]
pub enum Opcode {
    /// Create an object from a static map of values, as for var={'a': 3}.
    /// Any non-constant elements can be set afterwards with PutOwnByInd.
    /// Arg1 is the destination.
    /// Arg2 is a preallocation size hint.
    /// Arg3 is the number of static elements.
    /// Arg4 is the index in the object key buffer table.
    /// Arg5 is the index in the object val buffer table.
    NewObjectWithBuffer,
    NewObjectWithBufferLong,

    /// Create a new, empty Object using the built-in constructor (regardless of
    /// whether it was overridden).
    /// Arg1 = {}
    NewObject,

    /// Create a new empty Object with the specified parent. If the parent is
    /// null, no parent is used. If the parent is not an object, the builtin
    /// Object.prototype is used. Otherwise the parent itself is used.
    /// Arg1 = the created object
    /// Arg2 = the parent.
    NewObjectWithParent,

    /// Create an array from a static list of values, as for var=[1,2,3].
    /// Any non-constant elements can be set afterwards with PutOwnByIndex.
    /// Arg1 is the destination.
    /// Arg2 is a preallocation size hint.
    /// Arg3 is the number of static elements.
    /// Arg4 is the index in the array buffer table.
    NewArrayWithBuffer,
    NewArrayWithBufferLong,

    /// Create a new array of a given size.
    /// Arg1 = new Array(Arg2)
    NewArray,

    /// Arg1 = Arg2 (Register copy)
    Mov,

    /// Arg1 = Arg2 (Register copy, long index)
    MovLong,

    /// Arg1 = -Arg2 (Unary minus)
    Negate,

    /// Arg1 = !Arg2 (Boolean not)
    Not,

    /// Arg1 = ~Arg2 (Bitwise not)
    BitNot,

    /// Arg1 = typeof Arg2 (JS typeof)
    TypeOf,

    /// Arg1 = Arg2 == Arg3 (JS equality)
    Eq,

    /// Arg1 = Arg2 === Arg3 (JS strict equality)
    StrictEq,

    /// Arg1 = Arg2 != Arg3 (JS inequality)
    Neq,

    /// Arg1 = Arg2 !== Arg3 (JS strict inequality)
    StrictNeq,

    /// Arg1 = Arg2 < Arg3 (JS less-than)
    Less,

    /// Arg1 = Arg2 <= Arg3 (JS less-than-or-equals)
    LessEq,

    /// Arg1 = Arg2 > Arg3 (JS greater-than)
    Greater,

    /// Arg1 = Arg2 >= Arg3 (JS greater-than-or-equals)
    GreaterEq,

    /// Arg1 = Arg2 + Arg3 (JS addition/concatenation)
    Add,

    /// Arg1 = Arg2 + Arg3 (Numeric addition, skips number check)
    AddN,

    /// Arg1 = Arg2 * Arg3 (JS multiplication)
    Mul,

    /// Arg1 = Arg2 * Arg3 (Numeric multiplication, skips number check)
    MulN,

    /// Arg1 = Arg2 / Arg3 (JS division)
    Div,

    /// Arg1 = Arg2 / Arg3 (Numeric division, skips number check)
    DivN,

    /// Arg1 = Arg2 % Arg3 (JS remainder)
    Mod,

    /// Arg1 = Arg2 - Arg3 (JS subtraction)
    Sub,

    /// Arg1 = Arg2 - Arg3 (Numeric subtraction, skips number check)
    SubN,

    /// Arg1 = Arg2 << Arg3 (JS bitshift left)
    LShift,

    /// Arg1 = Arg2 >> Arg3 (JS signed bitshift right)
    Rshift,

    /// Arg1 = Arg2 >>> Arg3 (JS unsigned bitshift right)
    URShift,

    /// Arg1 = Arg2 & Arg3 (JS bitwise AND)
    BitAnd,

    /// Arg1 = Arg2 ^ Arg3 (JS bitwise XOR)
    BitXor,

    /// Arg1 = Arg2 | Arg3 (JS bitwise OR)
    BitOr,

    /// Check whether Arg2 contains Arg3 in its prototype chain.
    /// Note that this is not the same as JS instanceof.
    /// Pseudocode: Arg1 = prototypechain(Arg2).contains(Arg3)
    InstanceOf,

    /// Arg1 = Arg2 in Arg3 (JS relational 'in')
    IsIn,

    /// Get an environment (scope) from N levels up the stack.
    /// 0 is the current environment, 1 is the caller's environment, etc.
    GetEnvironment,

    /// Store a value in an environment.
    /// StoreNPToEnvironment[L] store a non-pointer value in an environment
    /// Arg1 is the environment (as fetched by GetEnvironment).
    /// Arg2 is the environment index slot number.
    /// Arg3 is the value.
    StoreToEnvironment,
    StoreToEnvironmentL,
    StoreNPToEnvironment,
    StoreNPToEnvironmentL,

    /// Load a value from an environment.
    /// Arg1 is the destination.
    /// Arg2 is the environment (as fetched by GetEnvironment).
    /// Arg3 is the environment index slot number.
    LoadFromEnvironment,
    LoadFromEnvironmentL,

    /// Get the global object (the object in which global variables are stored).
    GetGlobalObject,

    /// Obtain the value of NewTarget from the frame.
    /// Arg1 = NewTarget
    GetNewTarget,

    /// Create a new environment, to store values captured by closures.
    CreateEnvironment,

    /// Declare a global variable by string table index.
    /// The variable will be set to undefined.
    DeclareGlobalVar,

    /// Get an object property by string table index.
    /// Arg1 = Arg2[stringtable[Arg4]]
    /// Arg3 is a cache index used to speed up the above operation.
    GetByIdShort,
    GetById,
    GetByIdLong,

    /// Get an object property by string table index, or throw if not found.
    /// This is similar to GetById, but intended for use with global variables
    /// where Arg2 = GetGlobalObject.
    TryGetById,
    TryGetByIdLong,

    /// Set an object property by string index.
    /// Arg1[stringtable[Arg4]] = Arg2.
    PutById,
    PutByIdLong,

    /// Set an object property by string index, or throw if undeclared.
    /// This is similar to PutById, but intended for use with global variables
    /// where Arg1 = GetGlobalObject.
    TryPutById,
    TryPutByIdLong,

    /// Create a new own property on an object. This is similar to PutById, but
    /// the destination must be an object, it only deals with own properties,
    /// ignoring the prototype chain, and the property must not already be defined.
    /// Similarly to PutById, the property name cannot be a valid array index.
    /// Arg1 is the destination object, which is known to be an object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    PutNewOwnByIdShort,
    PutNewOwnById,
    PutNewOwnByIdLong,

    /// Create a new non-enumerable own property on an object. This is the same as
    /// PutNewOwnById, but creates the property with different enumerability.
    /// Arg1 is the destination object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    PutNewOwnNEById,
    PutNewOwnNEByIdLong,

    /// Assign a value to a constant integer own property which will be created as
    /// enumerable. This is used (potentially in conjunction with
    /// NewArrayWithBuffer) for arr=[foo,bar] initializations.
    /// Arg1[Arg3] = Arg2;
    PutOwnByIndex,
    PutOwnByIndexL,

    /// Set an own property identified by value.
    /// Arg1 is the destination object.
    /// Arg2 is the value to write.
    /// Arg3 is the property name.
    /// Arg4 : bool -> enumerable. If true, the property is created as enumerable,
    ///        non-enumerable otherwise.
    /// Arg1[Arg3] = Arg2;
    PutOwnByVal,

    /// Delete a property by string table index.
    /// Arg1 = delete Arg2[stringtable[Arg3]]
    DelById,
    DelByIdLong,

    /// Get a property by value. Constants string values should instead use GetById.
    /// Arg1 = Arg2[Arg3]
    GetByVal,

    /// Set a property by value. Constant string values should instead use GetById
    /// (unless they are array indices according to ES5.1 section 15.4, in which
    /// case this is still the right opcode).
    /// Arg1[Arg2] = Arg3
    PutByVal,

    /// Delete a property by value (when the value is not known at compile time).
    /// Arg1 = delete Arg2[Arg3]
    DelByVal,

    /// Add a getter and a setter for a property by value.
    /// Object.defineProperty(Arg1, Arg2, { get: Arg3, set: Arg4 }).
    /// Arg1 is the target object which will have a property defined.
    /// Arg2 is the property name
    /// Arg3 is the getter closure or undefined
    /// Arg4 is the setter closure or undefined
    /// Arg5 : boolean - if true, the property will be enumerable.
    PutOwnGetterSetterByVal,

    /// Get the list of properties from an object to implement for..in loop.
    /// Returns Arg1, which is the register that holds array of properties.
    /// Returns Undefined if the object is null/undefined.
    /// Arg2 is the register that holds the object.
    /// Arg3 is the register that holds the iterating index.
    /// Arg4 is the register that holds the size of the property list.
    GetPNameList,

    /// Get the next property in the for..in iterator.
    /// Returns Arg1, which is the next property. Undefined if unavailable.
    /// Arg2 is the register that holds array of properties.
    /// Arg3 is the register that holds the object.
    /// Arg4 is the register that holds the iterating index.
    /// Arg5 is the register that holds the size of the property list.
    GetNextPName,

    /// Call a function.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///      from the end of the current frame.
    Call,

    /// Call a constructor, with semantics identical to Call.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///      from the end of the current frame. The first argument 'this'
    ///      is assumed to be created with CreateThis.
    Construct,

    /// Call a function with one arg.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    Call1,

    /// Call a function directly without a closure.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the number of arguments, assumed to be found in reverse order
    ///      from the end of the current frame. The first argument 'this'
    ///      is assumed to be created with CreateThis.
    /// Arg3 is index in the function table.
    /// Note that we expect the variable-sized argument to be last.
    CallDirect,

    /// Call a function with two args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    Call2,

    /// Call a function with three args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    /// Arg5 is the third argument.
    Call3,

    /// Call a function with four args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    /// Arg5 is the third argument.
    /// Arg6 is the fourth argument.
    Call4,

    /// Identical to Call, but allowing more arguments.
    CallLong,

    /// Identical to Construct, but allowing more arguments.
    ConstructLong,

    /// Identical to CallDirect, but the function index is 32-bit.
    CallDirectLongIndex,

    /// Call a builtin function.
    /// Note this is NOT marked as a Ret target, because the callee is native
    /// and therefore never JS.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the builtin number.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///      from the end of the current frame.
    /// thisArg is set to "undefined".
    CallBuiltin,

    /// Return a value from the current function.
    /// return Arg1;
    Ret,

    /// Catch an exception (the first instruction in an exception handler).
    /// } catch(Arg1) {
    Catch,

    /// ES6 18.2.1.1 PerformEval(Arg2, evalRealm, strictCaller=true, direct=true)
    /// Arg1 is the destination of the return value.
    /// Arg2 is the value to eval.
    DirectEval,

    /// Throw an exception.
    /// throw Arg1;
    Throw,

    /// Throw ReferenceError if the operand is HermesValue::undefined
    ThrowIfUndefindedInst,

    /// Implementation dependent debugger action.
    Debugger,

    /// Fast check for an async interrupt request.
    AsyncBreakCheck,

    /// Define a profile point.
    /// Arg1 is the function local profile point index. The first one will have the
    /// largest index. If there are more than 2^16 profile points in the function,
    /// all the overflowed profile points have index zero.
    ProfilePoint,

    /// Unreachable opcode for stubs and similar.
    Unreachable,

    /// Create a closure.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    CreateClosure,
    CreateClosureLongIndex,

    /// Create a closure for a GeneratorFunction.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    CreateGeneratorClosure,
    CreateGeneratorClosureLongIndex,

    /// Allocate an empty, uninitialized object (immediately before a constructor).
    /// Arg1 is the destination register.
    /// Arg2 is the prototype to assign it.
    /// Arg3 is the constructor closure that will be used*.
    /// * To allow internal constructors to have special objects allocated.
    CreateThis,

    /// Choose the result of a constructor: 'this' or a returned object.
    /// Arg1 is the result.
    /// Arg2 is the 'this' object used for the constructor.
    /// Arg3 is the constructor's return value.
    /// Arg1 = Arg3 instanceof Object ? Arg3 : Arg2
    SelectObject,

    /// Load a function parameter by index. Starts at 0 with 'this'.
    /// Arg1 = Arg2 == 0 ? this : arguments[Arg2 - 1];
    LoadParam,

    /// Like LoadParam, but allows accessing arguments >= 255.
    LoadParamLong,

    /// Load a constant integer value.
    LoadConstUInt8,
    LoadConstInt,

    /// Load a constant double value
    LoadConstDouble,

    /// Load a constant string value by string table index.
    LoadConstString,
    LoadConstStringLongIndex,

    /// Load common constants.
    LoadConstUndefined,
    LoadConstNull,
    LoadConstTrue,
    LoadConstFalse,
    LoadConstZero,

    /// Coerce a value assumed to contain 'this' to an object using non-strict
    /// mode rules. Primitives are boxed, \c null or \c undefed produce the global
    /// object.
    /// Arg1 = coerce_to_object(Arg2)
    CoerceThisNS,

    /// Obtain the raw \c this value and coerce it to an object. Equivalent to:
    /// \code
    ///     LoadParam    Arg1, #0
    ///     CoerceThisNS Arg1, Arg1
    /// \endcode
    LoadThisNS,

    /// Convert a value to a number.
    /// Arg1 = Arg2 - 0
    ToNumber,

    /// Convert a value to a 32-bit signed integer.
    /// Arg1 = Arg2 | 0
    ToInt32,

    /// Convert a value to a string as if evaluating the expression:
    ///     Arg1 = "" + Arg2
    /// In practice this means
    ///     Arg1 = ToString(ToPrimitive(Arg2, PreferredType::NONE))
    /// with ToPrimitive (ES5.1 9.1) and ToString (ES5.1 9.8).
    AddEmptyString,

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
    GetArgumentsPropByVal,

    /// Get the length of the 'arguments' array.
    /// Arg1 is the result.
    /// Arg2 is the lazy loaded register.
    /// Arg1 = arguments.length
    GetArgumentsLength,

    /// Create a regular expression.
    /// Arg1 is the result.
    /// Arg2 is the string index of the pattern.
    /// Arg3 is the string index of the flags.
    /// Arg4 is the regexp bytecode index in the regexp table.
    CreateRegExp,

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
    SwitchImm,

    /// Start the generator by jumping to the next instruction to begin.
    /// Restore the stack frame if this generator has previously been suspended.
    StartGenerator,

    /// Resume generator by performing one of the following user-requested actions:
    /// - next(val): Set Arg1 to val, Arg2 to false, run next instruction
    /// - return(val): Set Arg1 to val, Arg2 to true, run next instruction
    /// - throw(val): Throw val as an error
    /// Arg1 is the result provided by the user.
    /// Arg2 is a boolean which is true if the user requested a return().
    ResumeGenerator,

    /// Set the generator status to complete, but do not return.
    CompleteGenerator,

    /// Create a generator.
    /// Arg1 is the register in which to store the generator.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    CreateGenerator,
    CreateGeneratorLongIndex,

    /// Arg1 [out] is the result iterator or index.
    /// Arg2 [in/out] is the source. Output for either the source or next method.
    /// If source is an array with an unmodified [Symbol.iterator], the result is
    /// 0. Else the result is source[Symbol.iterator] and the output is the .next()
    /// method on the iterator.
    /// See IR.md for IteratorBeginInst.
    IteratorBegin,

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
    IteratorNext,

    /// Arg1 [in] is the iterator or array index.
    /// Arg2 is a bool indicating whether to ignore the inner exception.
    /// If the iterator is an object, call iterator.return().
    /// If Arg2 is true, ignore exceptions which are thrown by iterator.return().
    /// See IR.md for IteratorCloseInst.
    IteratorClose,

    /// Unconditional branch to Arg1.
    Jmp,
    JmpLong,

    /// Conditional branches to Arg1 based on Arg2.
    JmpTrue,
    JmpTrueLong,
    JmpFalse,
    JmpFalseLong,

    /// Jump if the value is undefined.
    JmpUndefined,
    JmpUndefinedLong,

    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    SaveGenerator,
    SaveGeneratorLong,

    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.

    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    JLess,
    JLessLong,
    JNotLess,
    JNotLessLong,

    JLessN,
    JLessNLong,
    JNotLessN,
    JNotLessNLong,

    JLessEqual,
    JLessEqualLong,
    JNotLessEqual,
    JNotLessEqualLong,

    JLessEqualN,
    JLessEqualNLong,
    JNotLessEqualN,
    JNotLessEqualNLong,

    JGreater,
    JGreaterLong,
    JNotGreater,
    JNotGreaterLong,

    JGreaterN,
    JGreaterNLong,
    JNotGreaterN,
    JNotGreaterNLong,

    JGreaterEqual,
    JGreaterEqualLong,
    JNotGreaterEqual,
    JNotGreaterEqualLong,

    JGreaterEqualN,
    JGreaterEqualNLong,
    JNotGreaterEqualN,
    JNotGreaterEqualNLong,

    JEqual,
    JEqualLong,
    JNotEqual,
    JNotEqualLong,

    JStrictEqual,
    JStrictEqualLong,
    JStrictNotEqual,
    JStrictNotEqualLong,
}

impl TryFrom<u8> for Opcode {
    type Error = &'static str;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0..=178 => Ok(unsafe { std::mem::transmute(byte) }),
            _ => Err("Unknown Bytecode"),
        }
    }
}

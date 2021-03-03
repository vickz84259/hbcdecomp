use std::convert::TryFrom;

#[derive(Debug)]
#[repr(u8)]
pub enum Builtins {
    // Array
    ArrayIsArray = 0,

    // ArrayBuffer
    ArrayBufferIsView = 1,

    // Date
    DateUTC = 2,
    DateNow = 3,
    DateParse = 4,

    // JSON
    JSONParse = 5,
    JSONStringify = 6,

    // Math
    MathAbs = 7,
    MathAcos = 8,
    MathAsin = 9,
    MathAtan = 10,
    MathAtan2 = 11,
    MathCeil = 12,
    MathCos = 13,
    MathExp = 14,
    MathFloor = 15,
    MathHypot = 16,
    MathImul = 17,
    MathLog = 18,
    MathMax = 19,
    MathMin = 20,
    MathPow = 21,
    MathRandom = 22,
    MathRound = 23,
    MathSin = 24,
    MathSqrt = 25,
    MathTan = 26,
    MathTrunc = 27,

    // Object
    ObjectCreate = 28,
    ObjectDefineProperties = 29,
    ObjectDefineProperty = 30,
    ObjectFreeze = 31,
    ObjectGetOwnPropertyDescriptor = 32,
    ObjectGetOwnPropertyNames = 33,
    ObjectGetPrototypeOf = 34,
    ObjectIsExtensible = 35,
    ObjectIsFrozen = 36,
    ObjectKeys = 37,
    ObjectSeal = 38,

    // String
    StringFromCharCode = 39,

    // HermesBuiltin
    HermesBuiltinSilentSetPrototypeOf = 40,
    HermesBuiltinRequireFast = 41,
    HermesBuiltinGetTemplateObject = 42,
    HermesBuiltinEnsureObject = 43,
    HermesBuiltinThrowTypeError = 44,
    HermesBuiltinGeneratorSetDelegated = 45,
    HermesBuiltinCopyDataProperties = 46,
    HermesBuiltinCopyRestArgs = 47,
    HermesBuiltinArraySpread = 48,
    HermesBuiltinApply = 49,
    HermesBuiltinExportAll = 50,
    HermesBuiltinExponentiationOperator = 51,
}

impl TryFrom<u8> for Builtins {
    type Error = &'static str;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0..=51 => Ok(unsafe { std::mem::transmute(byte) }),
            _ => Err("Invalid builtin method index"),
        }
    }
}

impl From<Builtins> for &'static str {
    fn from(builtin: Builtins) -> Self {
        use Builtins::*;

        match builtin {
            ArrayIsArray => "Array.isArray",

            ArrayBufferIsView => "ArrayBuffer.isView",

            DateUTC => "Date.UTC",
            DateNow => "Date.now",
            DateParse => "Date.parse",

            JSONParse => "JSON.parse",
            JSONStringify => "JSON.stringify",

            MathAbs => "Math.abs",
            MathAcos => "Math.acos",
            MathAsin => "Math.asin",
            MathAtan => "Math.atan",
            MathAtan2 => "Math.atan2",
            MathCeil => "Math.ceil",
            MathCos => "Math.cos",
            MathExp => "Math.exp",
            MathFloor => "Math.floor",
            MathHypot => "Math.hypot",
            MathImul => "Math.imul",
            MathLog => "Math.log",
            MathMax => "Math.max",
            MathMin => "Math.min",
            MathPow => "Math.pow",
            MathRandom => "Math.random",
            MathRound => "Math.round",
            MathSin => "Math.sin",
            MathSqrt => "Math.sqrt",
            MathTan => "Math.tan",
            MathTrunc => "Math.trunc",

            ObjectCreate => "Object.create",
            ObjectDefineProperties => "Object.defineProperties",
            ObjectDefineProperty => "Object.defineProperty",
            ObjectFreeze => "Object.freeze",
            ObjectGetOwnPropertyDescriptor => "Object.getOwnPropertyDescriptor",
            ObjectGetOwnPropertyNames => "Object.getOwnPropertyNames",
            ObjectGetPrototypeOf => "Object.getPrototypeOf",
            ObjectIsExtensible => "Object.isExtensible",
            ObjectIsFrozen => "Object.isFrozen",
            ObjectKeys => "Object.keys",
            ObjectSeal => "Object.seal",

            StringFromCharCode => "String.fromCharCode",

            HermesBuiltinSilentSetPrototypeOf => "HermesBuiltin.silentSetPrototypeOf",
            HermesBuiltinRequireFast => "HermesBuiltin.requireFast",
            HermesBuiltinGetTemplateObject => "HermesBuiltin.getTemplateObject",
            HermesBuiltinEnsureObject => "HermesBuiltin.ensureObject",
            HermesBuiltinThrowTypeError => "HermesBuiltin.throwTypeError",
            HermesBuiltinGeneratorSetDelegated => "HermesBuiltin.generatorSetDelegated",
            HermesBuiltinCopyDataProperties => "HermesBuiltin.copyDataProperties",
            HermesBuiltinCopyRestArgs => "HermesBuiltin.copyRestArgs",
            HermesBuiltinArraySpread => "HermesBuiltin.arraySpread",
            HermesBuiltinApply => "HermesBuiltin.apply",
            HermesBuiltinExportAll => "HermesBuiltin.exportAll",
            HermesBuiltinExponentiationOperator => "HermesBuiltin.exponentiationOperator",
        }
    }
}

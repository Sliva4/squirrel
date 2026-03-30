use libffi::middle::Arg;

/// Imports
use crate::{CodeGenerator, Error, meta::{Signature, Value}};
use std::collections::HashMap;

/// Represents jitted code block
pub type JittedBlock = *const u8;

/// Represents function jit information
pub struct FunctionJitInfo {
    /// Represents jit info per signature
    pub signatures: HashMap<Signature, SignatureJitInfo>,

    /// Is this function jit eligible?
    pub is_jit_eligible: bool,
}

/// Function jit info implementation
impl FunctionJitInfo {
    /// Creates new function jit info
    pub fn new() -> Self {
        Self {
            is_jit_eligible: true,
            signatures: HashMap::new(),
        }
    }

    /// Records a function call for a given signature.
    /// Automatically creates a new SignatureJitInfo if it does not exist
    pub fn record(&mut self, sign: Signature) {
        // If function is jit eligible -> recording statistics
        if self.is_jit_eligible {
            // Retrieving signature stats
            let sign = self
                .signatures
                .entry(sign.clone())
                .or_insert(SignatureJitInfo {
                    call_stats: 0,
                    jitted: None,
                });

            // Incrementing calls statistics
            sign.call_stats += 1;
        }
    }

    /// Tries call function by signature
    pub fn try_call(&mut self, sign: Signature, args: Vec<Value>) -> Result<Value, Error> {
        // Checking function is jit eligieble
        if self.is_jit_eligible {
            // Retrieving signature stats
            let sign = self
                .signatures
                .entry(sign.clone())
                .or_insert(SignatureJitInfo {
                    call_stats: 0,
                    jitted: None,
                });

            // Trying to retrieve jitted code
            let jitted = match sign.jitted {
                /// If function got jit code => it's already hot, so we don't need to increment call stats now
                Some(block) => {
                    block
                },
                // Else, we need to increment call stats and try to jit function
                None => {
                    // Incrementing call stats
                    sign.call_stats += 1;

                    // Checking threshold: if call stats > threshold => trying to jit function
                    if sign.call_stats > consts::JIT_SIGN_THRESHOLD {
                        // Creating jit generator
                        let mut generator = CodeGenerator::new();

                        // Peerforming code generation

                    } else {
                        return Err(Error::NoJitCode)
                    }
                }
            };
        } else {
            Err(Error::NoJitEligible)
        }
    }
}

/// Represents specific function signature statistics and info
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SignatureJitInfo {
    /// Calls statistics
    pub call_stats: usize,

    /// Jitted block of specific signature function, if exists
    pub jitted: Option<JittedBlock>,
}

/// Signature jit info implementation
impl SignatureJitInfo {
    /// Performs call
}

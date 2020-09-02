pub mod queue_family;
pub use queue_family::QueueFamilyExt;
use vulkano::instance::debug::{MessageSeverity, MessageType};

pub const MESSAGE_SEVERITY_VERBOSE: MessageSeverity = MessageSeverity {
    verbose: true,
    information: false,
    warning: false,
    error: false,
};

pub const MESSAGE_SEVERITY_INFORMATION: MessageSeverity = MessageSeverity {
    verbose: false,
    information: true,
    warning: false,
    error: false,
};

pub const MESSAGE_SEVERITY_WARNING: MessageSeverity = MessageSeverity {
    verbose: false,
    information: false,
    warning: true,
    error: false,
};

pub const MESSAGE_SEVERITY_ERROR: MessageSeverity = MessageSeverity {
    verbose: false,
    information: false,
    warning: false,
    error: true,
};

pub const MESSAGE_TYPE_GENERAL: MessageType = MessageType {
    general: true,
    validation: false,
    performance: false,
};

pub const MESSAGE_TYPE_VALIDATION: MessageType = MessageType {
    general: false,
    validation: true,
    performance: false,
};

pub const MESSAGE_TYPE_PERFORMANCE: MessageType = MessageType {
    general: false,
    validation: false,
    performance: true,
};

pub fn message_severity_to_string(severity: MessageSeverity) -> &'static str {
    match severity {
        MESSAGE_SEVERITY_VERBOSE => "Verbose",
        MESSAGE_SEVERITY_INFORMATION => "Info",
        MESSAGE_SEVERITY_WARNING => "Warning",
        MESSAGE_SEVERITY_ERROR => "Error",
        _ => panic!("not implemented for non-single severities"),
    }
}

pub fn message_type_to_string(ty: MessageType) -> &'static str {
    match ty {
        MESSAGE_TYPE_GENERAL => "General",
        MESSAGE_TYPE_VALIDATION => "Validation",
        MESSAGE_TYPE_PERFORMANCE => "Performance",
        _ => panic!("not implemented for non-single types"),
    }
}

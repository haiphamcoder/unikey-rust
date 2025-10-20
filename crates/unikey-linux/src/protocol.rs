//! XIM protocol handling and message processing

use crate::{XimError, XimResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// XIM protocol version
pub const XIM_PROTOCOL_VERSION: u16 = 0x0002;

/// XIM message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum XimMessageType {
    // Connection messages
    Connect = 0x0001,
    ConnectReply = 0x0002,
    Disconnect = 0x0003,
    
    // Authentication messages
    AuthRequired = 0x0004,
    AuthReply = 0x0005,
    AuthNext = 0x0006,
    AuthSetup = 0x0007,
    AuthSetupReply = 0x0008,
    
    // IM server messages
    QueryExtension = 0x0009,
    QueryExtensionReply = 0x000A,
    ListExtensions = 0x000B,
    ListExtensionsReply = 0x000C,
    
    // IM client messages
    CreateIC = 0x000D,
    CreateICReply = 0x000E,
    DestroyIC = 0x000F,
    DestroyICReply = 0x0010,
    
    // Input context messages
    SetICValues = 0x0011,
    SetICValuesReply = 0x0012,
    GetICValues = 0x0013,
    GetICValuesReply = 0x0014,
    
    // Preedit messages
    PreeditStart = 0x0015,
    PreeditStartReply = 0x0016,
    PreeditDraw = 0x0017,
    PreeditDrawReply = 0x0018,
    PreeditCaret = 0x0019,
    PreeditCaretReply = 0x001A,
    PreeditDone = 0x001B,
    PreeditDoneReply = 0x001C,
    
    // Status messages
    StatusStart = 0x001D,
    StatusStartReply = 0x001E,
    StatusDraw = 0x001F,
    StatusDrawReply = 0x0020,
    StatusDone = 0x0021,
    StatusDoneReply = 0x0022,
    
    // Commit messages
    Commit = 0x0023,
    CommitReply = 0x0024,
    
    // Forward event messages
    ForwardEvent = 0x0025,
    ForwardEventReply = 0x0026,
    
    // Sync messages
    Sync = 0x0027,
    SyncReply = 0x0028,
    
    // Error messages
    Error = 0x0029,
    ErrorReply = 0x002A,
}

impl XimMessageType {
    /// Get message type from u16
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x0001 => Some(Self::Connect),
            0x0002 => Some(Self::ConnectReply),
            0x0003 => Some(Self::Disconnect),
            0x0004 => Some(Self::AuthRequired),
            0x0005 => Some(Self::AuthReply),
            0x0006 => Some(Self::AuthNext),
            0x0007 => Some(Self::AuthSetup),
            0x0008 => Some(Self::AuthSetupReply),
            0x0009 => Some(Self::QueryExtension),
            0x000A => Some(Self::QueryExtensionReply),
            0x000B => Some(Self::ListExtensions),
            0x000C => Some(Self::ListExtensionsReply),
            0x000D => Some(Self::CreateIC),
            0x000E => Some(Self::CreateICReply),
            0x000F => Some(Self::DestroyIC),
            0x0010 => Some(Self::DestroyICReply),
            0x0011 => Some(Self::SetICValues),
            0x0012 => Some(Self::SetICValuesReply),
            0x0013 => Some(Self::GetICValues),
            0x0014 => Some(Self::GetICValuesReply),
            0x0015 => Some(Self::PreeditStart),
            0x0016 => Some(Self::PreeditStartReply),
            0x0017 => Some(Self::PreeditDraw),
            0x0018 => Some(Self::PreeditDrawReply),
            0x0019 => Some(Self::PreeditCaret),
            0x001A => Some(Self::PreeditCaretReply),
            0x001B => Some(Self::PreeditDone),
            0x001C => Some(Self::PreeditDoneReply),
            0x001D => Some(Self::StatusStart),
            0x001E => Some(Self::StatusStartReply),
            0x001F => Some(Self::StatusDraw),
            0x0020 => Some(Self::StatusDrawReply),
            0x0021 => Some(Self::StatusDone),
            0x0022 => Some(Self::StatusDoneReply),
            0x0023 => Some(Self::Commit),
            0x0024 => Some(Self::CommitReply),
            0x0025 => Some(Self::ForwardEvent),
            0x0026 => Some(Self::ForwardEventReply),
            0x0027 => Some(Self::Sync),
            0x0028 => Some(Self::SyncReply),
            0x0029 => Some(Self::Error),
            0x002A => Some(Self::ErrorReply),
            _ => None,
        }
    }
}

/// XIM message header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XimMessageHeader {
    /// Message type
    pub message_type: XimMessageType,
    /// Message length (including header)
    pub length: u16,
    /// Major protocol version
    pub major_version: u8,
    /// Minor protocol version
    pub minor_version: u8,
}

impl XimMessageHeader {
    /// Create a new message header
    pub fn new(message_type: XimMessageType, data_length: u16) -> Self {
        Self {
            message_type,
            length: data_length + 8, // 8 bytes for header
            major_version: (XIM_PROTOCOL_VERSION >> 8) as u8,
            minor_version: (XIM_PROTOCOL_VERSION & 0xFF) as u8,
        }
    }

    /// Serialize header to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        bytes.extend_from_slice(&(self.message_type as u16).to_le_bytes());
        bytes.extend_from_slice(&self.length.to_le_bytes());
        bytes.push(self.major_version);
        bytes.push(self.minor_version);
        bytes
    }

    /// Deserialize header from bytes
    pub fn from_bytes(data: &[u8]) -> XimResult<Self> {
        if data.len() < 8 {
            return Err(XimError::XimProtocolError("Invalid message header length".to_string()));
        }

        let message_type = u16::from_le_bytes([data[0], data[1]]);
        let length = u16::from_le_bytes([data[2], data[3]]);
        let major_version = data[4];
        let minor_version = data[5];

        let message_type = XimMessageType::from_u16(message_type)
            .ok_or_else(|| XimError::XimProtocolError("Unknown message type".to_string()))?;

        Ok(Self {
            message_type,
            length,
            major_version,
            minor_version,
        })
    }
}

/// XIM message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XimMessage {
    /// Message header
    pub header: XimMessageHeader,
    /// Message data
    pub data: Vec<u8>,
}

impl XimMessage {
    /// Create a new XIM message
    pub fn new(message_type: XimMessageType, data: Vec<u8>) -> Self {
        let header = XimMessageHeader::new(message_type, data.len() as u16);
        Self { header, data }
    }

    /// Serialize message to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes();
        bytes.extend_from_slice(&self.data);
        bytes
    }

    /// Deserialize message from bytes
    pub fn from_bytes(data: &[u8]) -> XimResult<Self> {
        let header = XimMessageHeader::from_bytes(data)?;
        let data_length = header.length as usize - 8;
        
        if data.len() < header.length as usize {
            return Err(XimError::XimProtocolError("Incomplete message data".to_string()));
        }

        let message_data = data[8..8 + data_length].to_vec();
        
        Ok(Self {
            header,
            data: message_data,
        })
    }
}

/// XIM connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XimConnectionInfo {
    /// Client ID
    pub client_id: u32,
    /// Client name
    pub client_name: String,
    /// Client version
    pub client_version: u16,
    /// Authentication data
    pub auth_data: Vec<u8>,
    /// Connection time
    pub connection_time: std::time::SystemTime,
}

/// XIM input context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XimInputContext {
    /// Input context ID
    pub ic_id: u32,
    /// Client window
    pub client_window: u32,
    /// Focus window
    pub focus_window: u32,
    /// Input method
    pub input_method: String,
    /// Preedit attributes
    pub preedit_attributes: HashMap<String, String>,
    /// Status attributes
    pub status_attributes: HashMap<String, String>,
    /// Current buffer
    pub current_buffer: String,
    /// Cursor position
    pub cursor_position: usize,
}

/// XIM preedit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XimPreeditInfo {
    /// Input context ID
    pub ic_id: u32,
    /// Preedit string
    pub preedit_string: String,
    /// Cursor position
    pub cursor_position: usize,
    /// Caret position
    pub caret_position: usize,
    /// Chunk information
    pub chunks: Vec<XimPreeditChunk>,
}

/// XIM preedit chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XimPreeditChunk {
    /// Chunk text
    pub text: String,
    /// Chunk attributes
    pub attributes: HashMap<String, String>,
    /// Chunk start position
    pub start_position: usize,
    /// Chunk end position
    pub end_position: usize,
}

/// XIM status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XimStatusInfo {
    /// Input context ID
    pub ic_id: u32,
    /// Status string
    pub status_string: String,
    /// Status attributes
    pub status_attributes: HashMap<String, String>,
}

/// XIM protocol handler
pub struct XimProtocolHandler {
    /// Message handlers
    message_handlers: HashMap<XimMessageType, Box<dyn Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync>>,
}

impl XimProtocolHandler {
    /// Create a new protocol handler
    pub fn new() -> Self {
        Self {
            message_handlers: HashMap::new(),
        }
    }

    /// Register a message handler
    pub fn register_handler<F>(&mut self, message_type: XimMessageType, handler: F)
    where
        F: Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync + 'static,
    {
        self.message_handlers.insert(message_type, Box::new(handler));
    }

    /// Handle a message
    pub fn handle_message(&self, message: &XimMessage) -> XimResult<Option<XimMessage>> {
        if let Some(handler) = self.message_handlers.get(&message.header.message_type) {
            let response = handler(message)?;
            Ok(Some(response))
        } else {
            // No handler registered, return None
            Ok(None)
        }
    }

    /// Process incoming data
    pub fn process_data(&self, data: &[u8]) -> XimResult<Vec<XimMessage>> {
        let mut messages = Vec::new();
        let mut offset = 0;

        while offset < data.len() {
            if offset + 8 > data.len() {
                break; // Not enough data for header
            }

            let header = XimMessageHeader::from_bytes(&data[offset..])?;
            let message_length = header.length as usize;

            if offset + message_length > data.len() {
                break; // Not enough data for complete message
            }

            let message = XimMessage::from_bytes(&data[offset..offset + message_length])?;
            messages.push(message);
            offset += message_length;
        }

        Ok(messages)
    }
}

impl Default for XimProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

use crate::models::{KeyCode, KeyMappingConfig, Keyboard, KeyboardConfig, LightModeConfig};
use crate::modes::is_custom_mode;

const BUFFER_SIZE: usize = 65;
const KEY_MAP_BUFFERS_SIZE: usize = 9;
const CUSTOM_LIGHT_MODE_BUFFERS_SIZE: usize = 7;

/// Build all protocol buffers for a keyboard configuration
pub fn build_buffers(keyboard: &Keyboard, config: &KeyboardConfig) -> Result<Vec<Vec<u8>>, String> {
    let mut buffers = Vec::new();

    // Always send standard light buffer
    if let Some(light_config) = &config.light_mode {
        let standard_buffer = build_standard_light_buffer(light_config)?;
        buffers.push(standard_buffer);

        // If custom light mode, send custom light buffers
        let is_custom = is_custom_mode(light_config.mode_bit, keyboard.rgb);
        if is_custom && light_config.custom_colors.is_some() {
            let custom_buffers = build_custom_light_buffers(keyboard, light_config)?;
            buffers.extend(custom_buffers);
        }
    }

    // If key mapping is enabled, send key mapping buffers
    if keyboard.key_map_enabled {
        if let Some(key_mapping) = &config.key_mapping {
            let mapping_buffers = build_key_mapping_buffers(keyboard, key_mapping)?;
            buffers.extend(mapping_buffers);
        }
    }

    Ok(buffers)
}

/// Build standard light mode buffer (65 bytes)
fn build_standard_light_buffer(config: &LightModeConfig) -> Result<Vec<u8>, String> {
    let mut buffer = vec![0u8; BUFFER_SIZE];

    buffer[0] = 0x0a;
    buffer[1] = 0x01;
    buffer[2] = 0x01;
    buffer[3] = 0x02;
    buffer[4] = 0x29;
    buffer[5] = config.mode_bit;
    buffer[7] = config.animation;
    buffer[8] = config.brightness;

    if let Some(color) = &config.color {
        buffer[9] = color.r;
        buffer[10] = color.g;
        buffer[11] = color.b;
    }

    buffer[12] = if config.random_colors { 0x01 } else { 0x00 };
    buffer[13] = config.sleep;

    Ok(buffer)
}

/// Build custom light mode buffers (7 buffers of 65 bytes each)
fn build_custom_light_buffers(
    _keyboard: &Keyboard,
    config: &LightModeConfig,
) -> Result<Vec<Vec<u8>>, String> {
    let custom_colors = config
        .custom_colors
        .as_ref()
        .ok_or("Custom colors not provided")?;

    // Create full buffer for all RGB data (7 * 65 bytes)
    let mut led_full_buffer = vec![0u8; CUSTOM_LIGHT_MODE_BUFFERS_SIZE * BUFFER_SIZE];

    // Fill in RGB data for each key
    for per_key in custom_colors {
        let buffer_index = per_key.buffer_index as usize;
        let rgb_index = buffer_index * 3;

        if rgb_index + 2 < led_full_buffer.len() {
            led_full_buffer[rgb_index] = per_key.color.r;
            led_full_buffer[rgb_index + 1] = per_key.color.g;
            led_full_buffer[rgb_index + 2] = per_key.color.b;
        }
    }

    let mut buffers = Vec::new();
    let mut led_buffer_index = 0;

    for i in 0..CUSTOM_LIGHT_MODE_BUFFERS_SIZE {
        let mut buffer = vec![0u8; BUFFER_SIZE];

        buffer[0] = 0x0a;
        buffer[1] = CUSTOM_LIGHT_MODE_BUFFERS_SIZE as u8;
        buffer[2] = (i + 1) as u8;

        if i == 0 {
            buffer[3] = 0x03;
            buffer[4] = 0x7e;
            buffer[5] = 0x01;
        }

        // Copy RGB data into buffer
        let start_index = if i == 0 { 6 } else { 3 };
        for buffer_index in start_index..BUFFER_SIZE {
            if led_buffer_index < led_full_buffer.len() {
                buffer[buffer_index] = led_full_buffer[led_buffer_index];
                led_buffer_index += 1;
            }
        }

        buffers.push(buffer);
    }

    Ok(buffers)
}

/// Build key mapping buffers (9 buffers of 65 bytes each)
fn build_key_mapping_buffers(
    keyboard: &Keyboard,
    config: &KeyMappingConfig,
) -> Result<Vec<Vec<u8>>, String> {
    // Create full buffer for all key mappings (9 * 65 bytes)
    let mut map_full_buffer = vec![0u8; KEY_MAP_BUFFERS_SIZE * BUFFER_SIZE];

    // First, set default key codes from keyboard config
    for key in &keyboard.keys {
        let buffer_index = key.buffer_index as usize;
        let map_index = buffer_index * 4;

        if map_index + 3 < map_full_buffer.len() {
            set_buffer_key(&mut map_full_buffer[map_index..], key.key_code);
        }
    }

    // Then apply custom mappings
    for mapping in &config.mappings {
        let buffer_index = mapping.buffer_index as usize;
        let map_index = buffer_index * 4;

        if map_index + 3 < map_full_buffer.len() {
            set_buffer_key(&mut map_full_buffer[map_index..], mapping.key_code);
        }
    }

    let mut buffers = Vec::new();
    let mut map_buffer_index = 0;

    for i in 0..KEY_MAP_BUFFERS_SIZE {
        let mut buffer = vec![0u8; BUFFER_SIZE];

        buffer[0] = 0x0a;
        buffer[1] = KEY_MAP_BUFFERS_SIZE as u8;
        buffer[2] = (i + 1) as u8;

        if i == 0 {
            buffer[3] = 0x01;
            buffer[4] = 0xf8;
        }

        // Copy key mapping data into buffer
        let start_index = if i == 0 { 5 } else { 3 };
        for buffer_index in start_index..BUFFER_SIZE {
            if map_buffer_index < map_full_buffer.len() {
                buffer[buffer_index] = map_full_buffer[map_buffer_index];
                map_buffer_index += 1;
            }
        }

        buffers.push(buffer);
    }

    Ok(buffers)
}

/// Set key code in buffer (4 bytes, big-endian)
fn set_buffer_key(buffer: &mut [u8], key_code: KeyCode) {
    let key_code_u32 = key_code.to_u32();

    if key_code_u32 >= 0x01000000 {
        buffer[0] = ((key_code_u32 >> 24) & 0xff) as u8;
        buffer[1] = ((key_code_u32 >> 16) & 0xff) as u8;
        buffer[2] = ((key_code_u32 >> 8) & 0xff) as u8;
        buffer[3] = (key_code_u32 & 0xff) as u8;
    } else if key_code_u32 >= 0x010000 {
        buffer[0] = 0x00;
        buffer[1] = ((key_code_u32 >> 16) & 0xff) as u8;
        buffer[2] = ((key_code_u32 >> 8) & 0xff) as u8;
        buffer[3] = (key_code_u32 & 0xff) as u8;
    } else if key_code_u32 >= 0x0100 {
        buffer[0] = 0x00;
        buffer[1] = 0x00;
        buffer[2] = ((key_code_u32 >> 8) & 0xff) as u8;
        buffer[3] = (key_code_u32 & 0xff) as u8;
    } else {
        buffer[0] = 0x00;
        buffer[1] = 0x00;
        buffer[2] = 0x00;
        buffer[3] = (key_code_u32 & 0xff) as u8;
    }
}

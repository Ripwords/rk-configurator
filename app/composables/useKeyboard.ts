import { invoke } from "@tauri-apps/api/core";

export interface Keyboard {
  id: {
    vid: number;
    pid: number;
  };
  path: string;
  name: string;
  image_path: string;
  keys: Key[];
  key_map_enabled: boolean;
  light_enabled: boolean;
  rgb: boolean;
  top_left_x: number;
  top_left_y: number;
  bottom_right_x: number;
  bottom_right_y: number;
}

export interface Key {
  buffer_index: number;
  key_code: number;
  top_x: number;
  top_y: number;
  bottom_x: number;
  bottom_y: number;
}

export interface RgbColor {
  r: number;
  g: number;
  b: number;
}

export interface PerKeyColor {
  buffer_index: number;
  color: RgbColor;
}

export interface LightModeConfig {
  mode_bit: number;
  animation: number;
  brightness: number;
  color?: RgbColor;
  random_colors: boolean;
  sleep: number;
  custom_colors?: PerKeyColor[];
}

export interface KeyMapping {
  buffer_index: number;
  key_code: number;
}

export interface KeyMappingConfig {
  mappings: KeyMapping[];
}

export interface KeyboardConfig {
  light_mode?: LightModeConfig;
  key_mapping?: KeyMappingConfig;
}

export interface LightingMode {
  name: string;
  mode_bit: number;
}

export const useKeyboard = () => {
  const scanKeyboards = async (): Promise<Keyboard[]> => {
    return await invoke<Keyboard[]>("scan_keyboards");
  };

  const sendKeyboardConfig = async (
    keyboardPath: string,
    config: KeyboardConfig,
    keyboard: Keyboard
  ): Promise<void> => {
    return await invoke("send_keyboard_config", {
      keyboardPath,
      config,
      keyboard,
    });
  };

  const getLightingModes = async (isRgb: boolean): Promise<LightingMode[]> => {
    return await invoke<LightingMode[]>("get_lighting_modes", { isRgb });
  };

  return {
    scanKeyboards,
    sendKeyboardConfig,
    getLightingModes,
  };
};

<script setup lang="ts">
import { useKeyboard, type Keyboard } from "~/composables/useKeyboard";
import { convertFileSrc } from "@tauri-apps/api/core";

const { scanKeyboards } = useKeyboard();

const keyboards = ref<Keyboard[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

const refreshKeyboards = async () => {
  loading.value = true;
  error.value = null;
  try {
    keyboards.value = await scanKeyboards();
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Failed to scan keyboards";
    console.error("Failed to scan keyboards:", e);
  } finally {
    loading.value = false;
  }
};

const selectKeyboard = (keyboard: Keyboard) => {
  // Use hex format for VID and PID to match parsing in detail page
  const vidHex = keyboard.id.vid.toString(16).padStart(4, "0");
  const pidHex = keyboard.id.pid.toString(16).padStart(4, "0");
  navigateTo(`/keyboard/${vidHex}-${pidHex}`);
};

const getKeyboardImagePath = (keyboard: Keyboard) => {
  // Convert Tauri resource path to a URL that can be used in img src
  // The image_path from Rust is like "keyboards/258a/images/52.png"
  const resourcePath = keyboard.image_path;
  console.log("[RK-Configurator] Image path:", resourcePath);

  try {
    // In Tauri 2, convertFileSrc converts resource paths to accessible URLs
    // For bundled resources, it automatically uses the asset protocol
    const convertedPath = convertFileSrc(resourcePath);
    console.log("[RK-Configurator] Converted to:", convertedPath);
    return convertedPath;
  } catch (e) {
    console.error(
      "[RK-Configurator] Failed to convert image path:",
      e,
      keyboard.image_path
    );
    // Fallback: try constructing asset URL manually
    // In Tauri 2, resources are accessible via asset:// protocol
    return `asset://localhost/${resourcePath}`;
  }
};

const handleImageError = (event: Event) => {
  const img = event.target as HTMLImageElement;
  console.error("[RK-Configurator] Image failed to load:", img.src);
  img.style.display = "none";
};

onMounted(() => {
  refreshKeyboards();
});
</script>

<template>
  <div class="container mx-auto p-6">
    <div class="mb-6">
      <h1 class="text-3xl font-bold mb-2">Royal Kludge Configurator</h1>
      <p class="text-muted">Configure your Royal Kludge keyboard</p>
    </div>

    <UCard v-if="loading" class="mb-4">
      <div class="flex items-center justify-center p-8">
        <UIcon name="i-lucide-loader-2" class="w-6 h-6 animate-spin mr-2" />
        <span>Scanning for keyboards...</span>
      </div>
    </UCard>

    <UCard v-else-if="error" class="mb-4">
      <UAlert
        color="error"
        variant="soft"
        :title="error"
        icon="i-lucide-alert-circle"
      />
    </UCard>

    <div v-else-if="keyboards.length === 0" class="text-center py-12">
      <UIcon
        name="i-lucide-keyboard"
        class="w-16 h-16 mx-auto mb-4 text-muted"
      />
      <h2 class="text-xl font-semibold mb-2">No keyboards found</h2>
      <p class="text-muted mb-4">
        Connect a Royal Kludge keyboard and click refresh
      </p>
      <UButton @click="refreshKeyboards" icon="i-lucide-refresh-cw">
        Refresh
      </UButton>
    </div>

    <div v-else class="flex flex-col items-center gap-4 max-w-2xl mx-auto">
      <UCard
        v-for="keyboard in keyboards"
        :key="`${keyboard.id.vid}-${keyboard.id.pid}`"
        class="cursor-pointer hover:shadow-lg transition-shadow w-full"
        @click="selectKeyboard(keyboard)"
      >
        <template #header>
          <div class="flex items-center justify-between">
            <h3 class="font-semibold text-lg">{{ keyboard.name }}</h3>
            <UBadge
              :color="keyboard.rgb ? 'primary' : 'neutral'"
              variant="soft"
            >
              {{ keyboard.rgb ? "RGB" : "Single Color" }}
            </UBadge>
          </div>
        </template>

        <div class="space-y-4">
          <div
            class="aspect-video bg-muted rounded-lg flex items-center justify-center overflow-hidden p-4"
          >
            <img
              :src="getKeyboardImagePath(keyboard)"
              :alt="keyboard.name"
              class="max-w-full max-h-full object-contain"
              @error="handleImageError"
            />
          </div>

          <div class="text-sm space-y-1">
            <div class="flex justify-between">
              <span class="text-muted">VID:</span>
              <span class="font-mono"
                >0x{{ keyboard.id.vid.toString(16).padStart(4, "0") }}</span
              >
            </div>
            <div class="flex justify-between">
              <span class="text-muted">PID:</span>
              <span class="font-mono"
                >0x{{ keyboard.id.pid.toString(16).padStart(4, "0") }}</span
              >
            </div>
            <div class="flex justify-between">
              <span class="text-muted">Keys:</span>
              <span>{{ keyboard.keys.length }}</span>
            </div>
          </div>
        </div>

        <template #footer>
          <UButton
            block
            @click.stop="selectKeyboard(keyboard)"
            icon="i-lucide-settings"
          >
            Configure
          </UButton>
        </template>
      </UCard>
    </div>

    <div v-if="keyboards.length > 0" class="mt-6 flex justify-center">
      <UButton
        @click="refreshKeyboards"
        icon="i-lucide-refresh-cw"
        variant="outline"
      >
        Refresh Keyboards
      </UButton>
    </div>
  </div>
</template>

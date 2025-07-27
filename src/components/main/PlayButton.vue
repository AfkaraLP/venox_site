<template>
  <button
    class="mc-play-btn"
    :style="{
      width: size + 'px',
      height: size + 'px',
      fontSize: (size * 0.6) + 'px',
      background: bgColor,
      color: color
    }"
    @click="$emit('click')"
    :aria-label="ariaLabel"
  >
    ▶
  </button>
</template>

<script setup lang="ts">
defineProps({
  size: { type: Number, default: 44 },
  color: { type: String, default: '#fff' },
  bgColor: { type: String, default: 'rgba(var(--vnx-fg), 0.5)' }, // Minecraft brown
  ariaLabel: { type: String, default: 'Play' }
})
</script>

<style scoped>
.mc-play-btn {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  cursor: pointer;
  transition: background 0.2s, color 0.2s;
  z-index: 2;
  font-family: var(--font-vnx);
  user-select: none;
  border-radius: 8px;
  border: 2px solid #fff2;
  outline: none;
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(8px);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.mc-play-btn:hover {
  box-shadow: 0 0 12px rgba(255, 255, 255, 0.2), 0 0 4px rgba(255, 255, 255, 0.1) inset;
  transform: translate(-50%, -50%) scale(1.05);
}
</style> 

<template>
  <section class="home-hero">
    <div class="hero-bg"></div>
    <div class="hero-content">
      <!-- Removed avatar image -->
      <h1 class="channel-name">Venox</h1>
      <p class="tagline">Gaming • Genshin • Art • Music</p>
      <p class="desc">Welcome to Venox's world! Dive into epic games, magical art, and enchanting music~</p>
    </div>
  </section>
</template>

<style scoped>
.home-hero {
  position: relative;
  background: linear-gradient(120deg, #e0c3fc 0%, #8ec5fc 100%);
  padding: 3rem 0 2rem 0;
  text-align: center;
  overflow: hidden;
  border-radius: 0 0 2rem 2rem;
  box-shadow: 0 4px 24px rgba(120, 120, 200, 0.12);
}
.hero-bg {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: url('/src/assets/genshin_font.ttf') repeat;
  opacity: 0.04;
  z-index: 0;
}
.hero-content {
  position: relative;
  z-index: 1;
}
/* .avatar styles remain for future use if needed */
.channel-name {
  font-family: 'genshin', sans-serif;
  font-size: 2.5rem;
  color: #5a4fcf;
  margin: 0;
}
.tagline {
  font-size: 1.2rem;
  color: #7b6ee6;
  margin-bottom: 0.5rem;
}
.desc {
  color: #444;
  font-size: 1rem;
  margin-bottom: 0;
}
</style> 
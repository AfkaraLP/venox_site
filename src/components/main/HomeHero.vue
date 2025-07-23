<template>
  <section class="home-hero-mc">
    <div class="hero-bg-mc"></div>
    <div class="hero-content-mc">
      <h1 class="channel-name-mc">VENOX</h1>
      <p class="tagline-mc">Gaming • Genshin • Art • Music</p>
      <p class="desc-mc">Welcome to Venox's world! Dive into epic games, magical art, and enchanting music~</p>
    </div>
  </section>
</template>

<style scoped>
.home-hero-mc {
  position: relative;
  background: #bcbcbc;
  padding: 2.5rem 0 1.5rem 0;
  text-align: center;
  overflow: hidden;
  border-radius: 0;
  border: 6px double #222;
  box-shadow: 0 8px 0 #4e2e13, 0 12px 0 #bcbcbc;
  margin-bottom: 2rem;
}
.hero-bg-mc {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: repeating-linear-gradient(45deg, #fff2 0 8px, transparent 8px 16px), repeating-linear-gradient(-45deg, #0002 0 8px, transparent 8px 16px);
  opacity: 0.10;
  z-index: 0;
}
.hero-content-mc {
  position: relative;
  z-index: 1;
}
.channel-name-mc {
  font-family: var(--font-mc);
  font-size: 2.5rem;
  color: #4e2e13;
  margin: 0 0 0.5rem 0;
  text-shadow: 2px 2px 0 #fff, 0 0 4px #bcbcbc;
  text-transform: uppercase;
  letter-spacing: 2px;
}
.tagline-mc {
  font-size: 1.1rem;
  color: #6aac4b;
  margin-bottom: 0.5rem;
  font-family: var(--font-mc);
  text-shadow: 1px 1px 0 #fff, 0 0 2px #bcbcbc;
  text-transform: uppercase;
  letter-spacing: 1px;
}
.desc-mc {
  color: #222;
  font-size: 1rem;
  margin-bottom: 0;
  font-family: var(--font-mc);
  text-shadow: 1px 1px 0 #fff, 0 0 2px #bcbcbc;
}
</style> 
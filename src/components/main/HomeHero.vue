<template>
  <section class="home-hero-mc">
    <div class="hero-bg-mc"></div>
    <div class="hero-content-mc">
      <h1 class="channel-name-mc"><span class="channel-name-pink">Lil</span>Venox</h1>
      <p class="tagline-mc">Gaming • Genshin • Art • Music</p>
      <p class="desc-mc">Welcome to Venox's world! Dive into epic games, magical art, and enchanting music~</p>
    </div>
  </section>
</template>

<style scoped>
.home-hero-mc {
  position: relative;
  padding: 2.5rem 0 1.5rem 0;
  text-align: center;
  overflow: hidden;
  border-radius: 0;
}
.hero-bg-mc {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  opacity: 0.10;
  z-index: 0;
}
.hero-content-mc {
  position: relative;
  z-index: 1;
}
.channel-name-mc {
  font-family: var(--font-vnx);
  font-size: 2.5rem;
  color: rgb(var(--vnx-black));
  margin: 0 0 0.5rem 0;
}
.channel-name-pink {
  color: rgb(var(--vnx-pink));
}
.tagline-mc {
  font-size: 1.1rem;
  font-family: var(--font-vnx);
}
.desc-mc {
  color: #222;
  font-size: 1rem;
  margin-bottom: 0;
  font-family: var(--font-vnx);
}
</style> 

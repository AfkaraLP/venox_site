<template>
  <section class="home-hero">
    <div class="hero-content">
      <h1 class="channel-name">LilVenox</h1>
      <p class="tagline">
        Gaming <span class="pink">/</span> Genshin <span class="pink">/</span> Art <span class="pink">/</span> Music
      </p>
      <p class="desc"><i class="quote"> "Venox. Games. Art. Sound. All in one place."</i></p>
    </div>
  </section>
</template>

<style scoped>
.home-hero {
  position: relative;
  border-top: 4px solid rgb(var(--vnx-pink));
  border-bottom: 4px solid rgb(var(--vnx-black));
  padding: 0 0 1.5rem 0;
  text-align: center;
  overflow: hidden;
  border-radius: 0;
  background: rgb(var(--vnx-fg));
  color: rgb(var(--vnx-white));
  border-radius: 16px;
}
.hero-content {
  position: relative;
  z-index: 1;
}
.channel-name {
  padding-top: 2rem;
  font-family: var(--font-vnx);
  font-size: 2.5rem;
  background: -webkit-linear-gradient(
    rgb(var(--vnx-pink)),
    rgb(var(--vnx-highlight))    
  );
  background-clip: text;
  color: transparent;
  margin: 0 0 0.5rem 0;
}
.tagline {
  font-size: 1.1rem;
  font-family: var(--font-vnx);
}
.desc {
  font-size: 1rem;
  margin-bottom: 0;
  font-family: var(--font-vnx);
}
.pink {
  color: rgb(var(--vnx-pink));
}

.quote {
  padding: 0.1621rem 0.9rem 0.1621rem 0.1rem;
  background: #fff2;
  border-left: 2px solid rgb(var(--vnx-pink));
  transform: skew(15deg, 15deg);
}
</style> 

<template>
  <section class="home-hero">
    <div class="hero-content">
      <h1 class="channel-name">LilVenox</h1>
      <p class="tagline">
        Gaming <span class="pink">/</span> Genshin <span class="pink">/</span> Art <span class="pink">/</span> Music
      </p>
      <p class="desc"><i class="quote"> "Venox. Games. Art. Sound. All in one place."</i></p>
      <div class="cta-row">
        <a href="https://ko-fi.com/lilvenox" class="cta-link kofi-link" target="_blank" rel="noopener">
          <img src="https://cdn.jsdelivr.net/npm/simple-icons@v11/icons/kofi.svg" alt="Ko-fi" class="cta-icon" />
          <span class="cta-text">
            Commissions on Ko-fi!
            <small class="cta-sub">(or just... fuel the venox grind ✦)</small>
          </span>
        </a>
        <a href="https://discord.gg/tQGVzWGUu4" class="cta-link discord-link" target="_blank" rel="noopener">
          <img src="https://cdn.jsdelivr.net/npm/simple-icons@v11/icons/discord.svg" alt="Discord" class="cta-icon" />
          <span class="cta-text">
            Venox's Flicking Corner
            <small class="cta-sub">(the art discord ✦)</small>
          </span>
        </a>
      </div>
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
.cta-row {
  display: inline-flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 0.75rem;
  margin-top: 1.2rem;
}
.cta-link {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1.4rem;
  border-radius: 8px;
  text-decoration: none;
  font-family: var(--font-vnx);
  font-size: 1rem;
  font-weight: 600;
  transition: background 0.2s, transform 0.15s;
  border-width: 1.5px;
  border-style: solid;
}
.kofi-link {
  background: rgba(var(--vnx-pink), 0.12);
  border-color: rgb(var(--vnx-pink));
  color: rgb(var(--vnx-pink));
}
.kofi-link:hover {
  background: rgba(var(--vnx-pink), 0.22);
  transform: scale(1.04);
}
.discord-link {
  background: rgba(88, 101, 242, 0.15);
  border-color: #5865f2;
  color: #a3adff;
}
.discord-link:hover {
  background: rgba(88, 101, 242, 0.28);
  transform: scale(1.04);
}
.cta-icon {
  width: 22px;
  height: 22px;
}
.kofi-link .cta-icon {
  filter: invert(0.75) sepia(0.4) saturate(3) hue-rotate(300deg);
}
.discord-link .cta-icon {
  filter: invert(72%) sepia(43%) saturate(2500%) hue-rotate(210deg) brightness(100%) contrast(95%);
}
.cta-text {
  display: inline-flex;
  flex-direction: column;
  align-items: center;
  line-height: 1.15;
}
.cta-sub {
  font-size: 0.72rem;
  font-weight: 400;
  opacity: 0.8;
  margin-top: 0.15rem;
}
</style> 

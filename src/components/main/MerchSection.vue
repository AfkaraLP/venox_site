<template>
  <section id="merch" class="merch-feature" aria-labelledby="merch-title">
    <div class="merch-art" aria-hidden="false">
      <span class="merch-sticker">OFFICIAL MERCH</span>
      <span class="merch-split" aria-hidden="true"></span>
      <img
        :src="merchThumb"
        alt="Venox T-Shirt, back view with the official Venox print"
        fetchpriority="high"
      />
      <span class="merch-shine" aria-hidden="true"></span>
    </div>
    <div class="merch-copy">
      <p class="merch-eyebrow">THE VENOX MERCH STORE</p>
      <h2 id="merch-title">Wear the<br /><span>official Venox.</span></h2>
      <p class="merch-description">
        Venox shirts, hoodies, totes, mugs, stickers, magnets and whatever
        else the store stocks — official Venox merch with multiple color
        choices, printed on demand.
      </p>
      <a
        class="merch-button"
        href="https://lilvenox.dashery.com/"
        target="_blank"
        rel="noopener noreferrer"
      >
        <span>SHOP THE MERCH</span><span aria-hidden="true">↗</span>
      </a>
      <p class="merch-note">Product details, options and availability at Dashery.</p>
    </div>
  </section>
</template>

<script setup lang="ts">
import merchThumb from "../../assets/images/venox-shirt-back.png"
</script>

<style scoped>
.merch-feature {
  position: relative;
  isolation: isolate;
  overflow: hidden;
  display: grid;
  grid-template-columns: minmax(0, 0.9fr) minmax(0, 1.1fr);
  align-items: center;
  gap: clamp(1.5rem, 4vw, 4rem);
  min-height: 440px;
  margin: 0 auto 2.5rem;
  padding: clamp(1.25rem, 4vw, 3.75rem);
  border: 1px solid rgba(255, 255, 255, 0.18);
  border-radius: 30px;
  color: #fff8fb;
  background:
    radial-gradient(ellipse at 87% 0%, rgba(255, 63, 193, .25), transparent 45%),
    linear-gradient(305deg, #17131c 0%, #241a2b 45%, #3a1130 100%);
  box-shadow: 0 24px 70px rgba(104, 18, 80, .3), inset 0 1px 0 rgba(255,255,255,.3);
}
.merch-feature::before {
  content: '';
  position: absolute;
  z-index: -1;
  width: 430px;
  height: 430px;
  left: -120px;
  bottom: -210px;
  border: 1px solid rgba(255,255,255,.2);
  border-radius: 50%;
  box-shadow: 0 0 0 34px rgba(255,255,255,.035), 0 0 0 72px rgba(255,255,255,.025);
}
.merch-art {
  position: relative;
  display: grid;
  place-items: center;
  min-height: 340px;
  overflow: hidden;
  border-radius: 24px;
  background: #0d0a12;
  box-shadow: 0 20px 45px rgba(49, 9, 35, .28), inset 0 0 0 1px rgba(255,255,255,.35);
  transform: rotate(1.5deg);
  transition: transform .25s ease, box-shadow .25s ease;
}
.merch-art:hover { transform: rotate(0) scale(1.015); box-shadow: 0 24px 55px rgba(49, 9, 35, .35); }
/* soft magenta glow backdrop so the black tee stands out */
.merch-split {
  position: absolute;
  inset: 0;
  background:
    radial-gradient(ellipse at 32% 38%, rgba(255, 63, 193, .5), transparent 62%),
    radial-gradient(ellipse at 75% 80%, rgba(104, 18, 80, .45), transparent 65%),
    #16101d;
}
.merch-art img {
  position: relative;
  width: 86%;
  max-width: 460px;
  filter: drop-shadow(0 18px 22px rgba(0, 0, 0, .45));
}
.merch-shine { position: absolute; inset: -45% auto -45% -50%; width: 30%; transform: rotate(20deg); background: linear-gradient(90deg, transparent, rgba(255,255,255,.35), transparent); animation: shine 5s ease-in-out infinite; pointer-events: none; }
@keyframes shine { 0%, 55% { left: -50%; } 85%, 100% { left: 130%; } }
.merch-sticker { position: absolute; z-index: 1; top: 18px; left: 18px; padding: .55rem .8rem; border: 1px solid rgba(255, 255, 255, .8); border-radius: 999px; color: #8e2858; background: rgba(255, 255, 255, .88); font-size: .72rem; font-weight: 800; letter-spacing: .1em; box-shadow: 0 5px 15px rgba(97,38,69,.12); }
.merch-copy { max-width: 520px; }
.merch-eyebrow { margin: 0 0 .85rem; color: #ff8fd2; font-size: .82rem; font-weight: 800; letter-spacing: .19em; }
.merch-copy h2 { margin: 0; font-size: clamp(2.2rem, 4.6vw, 4.2rem); line-height: .98; letter-spacing: -.055em; text-shadow: 0 4px 18px rgba(72,15,49,.22); }
.merch-copy h2 span { color: #ff9fd8; }
.merch-description { max-width: 460px; margin: 1.4rem 0 1.6rem; color: rgba(255,248,251,.9); font-size: clamp(1rem, 1.5vw, 1.15rem); line-height: 1.65; }
.merch-button { display: inline-flex; align-items: center; justify-content: space-between; gap: 1.25rem; min-height: 58px; padding: 0 1.35rem; border: 1px solid rgba(255, 63, 193, .9); border-radius: 999px; color: #1b0d18; background: linear-gradient(180deg, #ffb9e4 0%, #ff3fc1 100%); box-shadow: 0 8px 0 #a11d78, 0 15px 28px rgba(53,10,34,.28), inset 0 1px 0 #fff; font-weight: 900; letter-spacing: .04em; text-decoration: none; transition: transform .15s ease, box-shadow .15s ease; }
.merch-button:hover { transform: translateY(-3px); box-shadow: 0 11px 0 #a11d78, 0 20px 34px rgba(53,10,34,.32), inset 0 1px 0 #fff; }
.merch-button span:last-child { font-size: 1.4rem; }
.merch-note { margin: 1.1rem 0 0; color: rgba(255,255,255,.73); font-size: .83rem; }
@media (max-width: 760px) {
  .merch-feature { grid-template-columns: 1fr; gap: 1.4rem; min-height: auto; padding: 1rem 1rem 1.7rem; border-radius: 23px; }
  .merch-art { min-height: 260px; max-height: 360px; transform: none; }
  .merch-art img { max-height: 320px; }
  .merch-copy { padding: 0 .35rem; }
  .merch-copy h2 { font-size: clamp(2.4rem, 10.5vw, 3.4rem); }
  .merch-description { margin: 1rem 0 1.3rem; }
}
@media (prefers-reduced-motion: reduce) {
  .merch-shine { animation: none; }
  .merch-art, .merch-button { transition: none; }
}
</style>

<template>
  <section id="shop" class="shop-feature" aria-labelledby="shop-title">
    <a
      class="shop-art"
      href="https://www.plushshop.com/products/venoxmeow"
      target="_blank"
      rel="noopener noreferrer"
      aria-label="View the Venoxmeow plush on Plushshop (opens in a new tab)"
    >
      <span class="shop-sticker" aria-hidden="true">OFFICIAL COLLAB</span>
      <img
        src="https://www.plushshop.com/cdn/shop/files/venoxmeow-pre-order-ship-by-nov-15-dec-wan-ou-375.webp?v=1791303447&width=1100"
        alt="Venoxmeow plush product"
        fetchpriority="high"
      />
      <span class="shop-shine" aria-hidden="true"></span>
    </a>
    <div class="shop-copy">
      <p class="shop-eyebrow">THE VENoxMEOW PLUSH</p>
      <h1 id="shop-title">Meet your new<br /><span>favorite meow.</span></h1>
      <p class="shop-description">
        Venoxmeow — a plush personally designed by Venox’s dad himself.
        See the product and current details at Plushshop.
      </p>
      <a
        class="shop-button"
        href="https://www.plushshop.com/products/venoxmeow"
        target="_blank"
        rel="noopener noreferrer"
      >
        <span>GET YOUR VENOXMEOW</span><span aria-hidden="true">↗</span>
      </a>
      <p class="shop-note">Product details, options and availability on Plushshop.</p>
    </div>
  </section>
</template>

<style scoped>
.shop-feature {
  position: relative;
  isolation: isolate;
  overflow: hidden;
  display: grid;
  grid-template-columns: minmax(0, 1.1fr) minmax(300px, 0.9fr);
  align-items: center;
  gap: clamp(1.5rem, 4vw, 4rem);
  min-height: 490px;
  margin: 0 auto 2.5rem;
  padding: clamp(1.25rem, 4vw, 3.75rem);
  border: 1px solid rgba(255, 255, 255, 0.18);
  border-radius: 30px;
  color: #fff8fb;
  background:
    radial-gradient(ellipse at 13% 0%, rgba(255, 191, 222, .3), transparent 45%),
    linear-gradient(125deg, #a32962 0%, #d44c88 45%, #692a77 100%);
  box-shadow: 0 24px 70px rgba(213, 65, 130, .27), inset 0 1px 0 rgba(255,255,255,.3);
}
.shop-feature::before {
  content: '';
  position: absolute;
  z-index: -1;
  width: 430px;
  height: 430px;
  right: -120px;
  top: -210px;
  border: 1px solid rgba(255,255,255,.2);
  border-radius: 50%;
  box-shadow: 0 0 0 34px rgba(255,255,255,.035), 0 0 0 72px rgba(255,255,255,.025);
}
.shop-art {
  position: relative;
  display: grid;
  place-items: center;
  min-height: 370px;
  overflow: hidden;
  border-radius: 24px;
  background: linear-gradient(145deg, #ffe8f2, #f8c8df 55%, #fff5fb);
  box-shadow: 0 20px 45px rgba(49, 9, 35, .28), inset 0 0 0 1px rgba(255,255,255,.9);
  transform: rotate(-1.5deg);
  transition: transform .25s ease, box-shadow .25s ease;
}
.shop-art:hover { transform: rotate(0) scale(1.015); box-shadow: 0 24px 55px rgba(49, 9, 35, .35); }
.shop-art img { width: 100%; height: 100%; max-height: 460px; object-fit: contain; filter: drop-shadow(0 20px 18px rgba(84, 26, 58, .2)); }
.shop-shine { position: absolute; inset: -45% auto -45% -50%; width: 30%; transform: rotate(20deg); background: linear-gradient(90deg, transparent, rgba(255,255,255,.45), transparent); animation: shine 5s ease-in-out infinite; pointer-events: none; }
@keyframes shine { 0%, 55% { left: -50%; } 85%, 100% { left: 130%; } }
.shop-sticker { position: absolute; z-index: 1; top: 18px; left: 18px; padding: .55rem .8rem; border: 1px solid rgba(255,255,255,.8); border-radius: 999px; color: #8e2858; background: rgba(255,255,255,.83); font-size: .72rem; font-weight: 800; letter-spacing: .1em; box-shadow: 0 5px 15px rgba(97,38,69,.12); }
.shop-copy { max-width: 500px; }
.shop-eyebrow { margin: 0 0 .85rem; color: #ffd7e9; font-size: .82rem; font-weight: 800; letter-spacing: .19em; }
.shop-copy h1 { margin: 0; font-size: clamp(2.6rem, 5.2vw, 5rem); line-height: .98; letter-spacing: -.055em; text-shadow: 0 4px 18px rgba(72,15,49,.22); }
.shop-copy h1 span { color: #ffd9ec; }
.shop-description { max-width: 420px; margin: 1.4rem 0 1.6rem; color: rgba(255,248,251,.9); font-size: clamp(1rem, 1.5vw, 1.2rem); line-height: 1.65; }
.shop-button { display: inline-flex; align-items: center; justify-content: space-between; gap: 1.25rem; min-height: 58px; padding: 0 1.35rem; border: 1px solid rgba(255,255,255,.95); border-radius: 999px; color: #7b2250; background: linear-gradient(180deg, #fff 0%, #ffe3ef 100%); box-shadow: 0 8px 0 #b23c72, 0 15px 28px rgba(53,10,34,.28), inset 0 1px 0 #fff; font-weight: 900; letter-spacing: .04em; text-decoration: none; transition: transform .15s ease, box-shadow .15s ease; }
.shop-button:hover { transform: translateY(-3px); box-shadow: 0 11px 0 #b23c72, 0 20px 34px rgba(53,10,34,.32), inset 0 1px 0 #fff; }
.shop-button span:last-child { font-size: 1.4rem; }
.shop-note { margin: 1.1rem 0 0; color: rgba(255,255,255,.73); font-size: .83rem; }
@media (max-width: 760px) {
  .shop-feature { grid-template-columns: 1fr; gap: 1.4rem; min-height: auto; padding: 1rem 1rem 1.7rem; border-radius: 23px; }
  .shop-art { min-height: 280px; max-height: 340px; transform: none; }
  .shop-art img { max-height: 340px; }
  .shop-copy { padding: 0 .35rem; }
  .shop-copy h1 { font-size: clamp(2.75rem, 12vw, 4rem); }
  .shop-description { margin: 1rem 0 1.3rem; }
}
@media (prefers-reduced-motion: reduce) {
  .shop-shine { animation: none; }
  .shop-art, .shop-button { transition: none; }
}
</style>

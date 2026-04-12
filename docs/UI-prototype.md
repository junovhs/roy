<!DOCTYPE html>
<html lang="en"><head><meta charset="UTF-8"><title>Roy</title>
<link href="https://fonts.googleapis.com/css2?family=Geist:wght@300;400;500;600&family=JetBrains+Mono:wght@300;400;500&family=Fraunces:ital,opsz,wght@0,9..144,300;1,9..144,300;1,9..144,400&display=swap" rel="stylesheet">
<style>
:root{--surface:#16171a;--surface-2:#1c1d21;--surface-3:#24262b;--line:rgba(255,255,255,.06);--line-2:rgba(255,255,255,.1);--mint:#a8c5b4;--coral:#e87858;--coral-soft:#f09a7e;--peach:#e8b494;--ink:#e6e4df;--ink-dim:#9b9892;--ink-faint:#5f5d58;--neon:rgba(232,120,88,.3)}
*{margin:0;padding:0;box-sizing:border-box}
html,body{height:100%;background:linear-gradient(180deg,#28292d 0%,#1a1b1e 100%);color:var(--ink);font-family:'Geist','Inter',-apple-system,sans-serif;font-size:13px;overflow:hidden;-webkit-font-smoothing:antialiased;padding:24px}
.app{display:flex;flex-direction:column;height:calc(100vh - 48px);position:relative;z-index:2;background:linear-gradient(180deg,#1e1f23 0%,#16171a 100%);border-radius:14px;border:1px solid rgba(255,255,255,.08);box-shadow:0 30px 80px rgba(0,0,0,.5),0 0 0 1px rgba(0,0,0,.4),inset 0 1px 0 rgba(255,255,255,.04);overflow:hidden}
.app::before{content:'';position:absolute;top:0;left:0;right:0;height:28px;background:linear-gradient(180deg,rgba(255,255,255,.025),transparent);pointer-events:none;z-index:5;border-radius:14px 14px 0 0}
.tl-dots{position:absolute;top:14px;left:16px;display:flex;gap:7px;z-index:6}
.tl-dots span{width:11px;height:11px;border-radius:50%;background:rgba(255,255,255,.08);border:1px solid rgba(255,255,255,.04)}
.marquee{padding:18px 28px 14px 70px;display:flex;align-items:center;gap:20px;border-bottom:1px solid var(--line);position:relative;z-index:4}
.brand-mark{font-family:'Fraunces',serif;font-size:24px;font-weight:300;font-style:italic;color:var(--ink);line-height:1;letter-spacing:-.01em}
.session-strip{flex:1;display:flex;align-items:center;gap:22px;font-family:'Geist',sans-serif;font-size:11px;color:var(--ink-faint);padding-left:22px;border-left:1px solid var(--line)}
.strip-item{display:flex;align-items:baseline;gap:7px}
.strip-k{font-size:10px;color:var(--ink-faint);font-weight:400}
.strip-v{font-size:11px;color:var(--ink);font-weight:400}
.session-health{margin-left:auto;display:flex;align-items:center;gap:9px;padding:6px 13px;border:1px solid var(--line);border-radius:100px;background:rgba(255,255,255,.02)}
.heart{width:6px;height:6px;border-radius:50%;background:var(--mint);box-shadow:0 0 8px rgba(168,197,180,.4);animation:heart 2.4s ease-in-out infinite}
@keyframes heart{0%,100%{opacity:1;transform:scale(1)}50%{opacity:.5;transform:scale(.85)}}
.health-text{font-family:'Geist',sans-serif;font-size:11.5px;color:var(--ink-dim);font-weight:400}
.status-chip.needs-attention{border-color:rgba(255,184,154,.35);background:rgba(255,184,154,.06)}
.status-chip.needs-attention .heart{background:var(--peach);box-shadow:0 0 10px var(--peach)}
.status-chip.needs-attention .health-text{color:var(--peach)}
.status-chip.review{border-color:rgba(255,107,91,.35);background:rgba(255,107,91,.06)}
.status-chip.review .heart{background:var(--coral);box-shadow:0 0 10px var(--coral)}
.status-chip.review .health-text{color:var(--coral-soft)}
.status-chip.failed{border-color:rgba(232,96,90,.4);background:rgba(232,96,90,.08)}
.status-chip.failed .heart{background:#e8605a;box-shadow:0 0 10px #e8605a}
.status-chip.failed .health-text{color:#f08880}
.status-chip.exited .heart{background:var(--ink-faint);box-shadow:none;animation:none}
.status-chip.exited .health-text{color:var(--ink-faint)}
.main{flex:1;display:flex;position:relative;min-height:0;padding:20px 28px 12px}
.canvas{flex:1;display:flex;min-height:0;position:relative}
.pod{flex:1;position:relative;display:flex;flex-direction:column;min-width:0}
.pod-frame{flex:1;background:var(--surface);border-radius:10px;padding:0;position:relative;border:1px solid var(--line);box-shadow:0 20px 50px rgba(0,0,0,.3),inset 0 1px 0 rgba(255,255,255,.03);display:flex;flex-direction:column;min-height:0;overflow:hidden}
.pod-frame::before,.rivet{display:none}
.terminal{flex:1;background:transparent;border-radius:inherit;border:none;box-shadow:none;display:flex;flex-direction:column;overflow:hidden;position:relative}
.terminal::after,.terminal::before{display:none}
.terminal-head{padding:14px 22px 12px;display:flex;align-items:center;gap:10px;border-bottom:1px solid var(--line);position:relative;z-index:2}
.tdot{width:6px;height:6px;border-radius:50%;background:var(--ink-faint);opacity:.5}
.term-label{font-family:'JetBrains Mono',monospace;font-size:10px;color:var(--ink-faint);margin-left:6px;letter-spacing:.02em}
.term-label em{color:var(--ink-dim);font-style:normal}
.tb{flex:1;padding:22px 28px;font-family:'JetBrains Mono',monospace;font-size:12px;line-height:1.75;overflow-y:auto;color:var(--ink-dim);position:relative;z-index:2}
.tb::-webkit-scrollbar{width:5px}.tb::-webkit-scrollbar-thumb{background:rgba(255,255,255,.05);border-radius:2px}
.line{display:block}
.dim{color:var(--ink-faint)}.mint{color:var(--mint)}.coral{color:var(--coral-soft)}.peach{color:var(--peach)}.cream{color:#c8b89b}.prompt{color:var(--coral)}
.cursor{display:inline-block;width:7px;height:13px;background:var(--coral);box-shadow:0 0 6px var(--neon);vertical-align:text-bottom;margin-left:3px;animation:blink 1.1s step-end infinite}
@keyframes blink{50%{opacity:0}}
.edge{position:absolute;right:14px;top:50%;transform:translateY(-50%);display:flex;flex-direction:column;gap:6px;z-index:3}
.eb{width:30px;height:30px;border-radius:8px;background:var(--surface-2);border:1px solid var(--line);color:var(--ink-faint);cursor:pointer;display:flex;align-items:center;justify-content:center;font-family:'Geist',sans-serif;font-size:12px;font-weight:500;position:relative;transition:all .2s}
.eb:hover{color:var(--ink);border-color:var(--line-2);background:var(--surface-3)}
.eb.active{color:var(--coral);border-color:rgba(232,120,88,.35);background:rgba(232,120,88,.06)}
.eb .badge{position:absolute;top:-2px;right:-2px;width:7px;height:7px;border-radius:50%;background:var(--coral);box-shadow:0 0 6px var(--neon);display:none}
.eb.has-alert .badge{display:block;animation:heart 1.8s ease-in-out infinite}
.eb .tip{position:absolute;right:calc(100% + 10px);top:50%;transform:translateY(-50%);background:var(--surface-3);border:1px solid var(--line-2);padding:4px 10px;border-radius:5px;font-family:'Geist',sans-serif;font-size:10px;color:var(--ink-dim);white-space:nowrap;opacity:0;pointer-events:none;transition:opacity .2s}
.eb:hover .tip{opacity:1}
.drawer{position:absolute;top:20px;right:28px;bottom:20px;width:380px;background:var(--surface-2);backdrop-filter:blur(20px);border:1px solid var(--line-2);border-radius:10px;transform:translateX(calc(100% + 60px));transition:transform .4s cubic-bezier(.32,.72,0,1);display:flex;flex-direction:column;z-index:20;box-shadow:0 30px 60px rgba(0,0,0,.4)}
.drawer.open{transform:translateX(0)}
.dh{padding:20px 22px 14px;border-bottom:1px solid var(--line);display:flex;align-items:baseline;justify-content:space-between}
.dt{font-family:'Fraunces',serif;font-style:italic;font-weight:300;font-size:22px;color:var(--ink)}
.ds{font-family:'Geist',sans-serif;font-size:10px;color:var(--ink-faint);font-weight:400;margin-bottom:2px}
.dc{background:none;border:none;color:var(--ink-faint);cursor:pointer;font-size:20px;line-height:1}
.dc:hover{color:var(--ink)}
.db{flex:1;overflow-y:auto;padding:16px 22px 20px}
.db::-webkit-scrollbar{width:5px}.db::-webkit-scrollbar-thumb{background:rgba(255,255,255,.05);border-radius:2px}
.card{padding:14px 16px;margin-bottom:10px;border:1px solid var(--line);border-radius:9px;background:rgba(255,255,255,.015);transition:all .2s}
.card:hover{border-color:var(--line-2);background:rgba(255,255,255,.03)}
.card.urgent{border-color:rgba(232,120,88,.25);background:rgba(232,120,88,.04)}
.cl{font-family:'Geist',sans-serif;font-size:10px;color:var(--ink-faint);margin-bottom:6px;font-weight:400}
.card.urgent .cl{color:var(--coral-soft)}
.ct{font-family:'Fraunces',serif;font-style:italic;font-weight:300;font-size:17px;color:var(--ink);line-height:1.3;margin-bottom:7px}
.cb{font-size:11.5px;color:var(--ink-dim);line-height:1.55;margin-bottom:12px}
.cb code{font-family:'JetBrains Mono',monospace;font-size:10.5px;background:rgba(255,255,255,.04);padding:1px 5px;border-radius:3px;color:var(--ink)}
.ca{display:flex;gap:5px;flex-wrap:wrap}
.btn{font-family:'Geist',sans-serif;font-size:11px;padding:6px 12px;border:1px solid var(--line-2);background:transparent;color:var(--ink-dim);border-radius:6px;cursor:pointer;transition:all .15s;font-weight:400}
.btn:hover{color:var(--ink);border-color:rgba(255,255,255,.2)}
.btn.primary{background:var(--coral);color:#1a1b1e;border-color:var(--coral);font-weight:500}
.btn.primary:hover{background:var(--coral-soft);border-color:var(--coral-soft)}
.btn.ghost:hover{color:var(--peach);border-color:rgba(232,180,148,.3)}
.fl{display:flex;justify-content:space-between;padding:9px 0;border-bottom:1px solid var(--line);font-size:11px}
.fl:last-child{border:none}
.fn{font-family:'JetBrains Mono',monospace;color:var(--ink);font-size:11px}
.fm{font-family:'JetBrains Mono',monospace;color:var(--ink-faint);font-size:10px}
.ev{display:flex;gap:14px;padding:10px 0;border-bottom:1px solid var(--line)}
.ev:last-child{border:none}
.et{font-family:'JetBrains Mono',monospace;color:var(--ink-faint);font-size:10px;min-width:40px;padding-top:1px}
.ed{font-family:'Geist',sans-serif;font-size:12px;color:var(--ink-dim);flex:1;line-height:1.45}
.ed strong{color:var(--ink);font-weight:500}
.diag{display:flex;justify-content:space-between;padding:7px 0;font-family:'JetBrains Mono',monospace;font-size:10.5px;border-bottom:1px solid var(--line)}
.dk{color:var(--ink-faint)}.dv{color:var(--ink)}
.cw{padding:10px 28px 22px;flex-shrink:0}
.composer{width:100%;background:var(--surface-2);border:1px solid var(--line);border-radius:10px;padding:12px 14px;transition:all .2s;display:flex;flex-direction:column;gap:8px}
.composer:focus-within{border-color:var(--line-2);background:var(--surface-3)}
.ci{background:none;border:none;outline:none;color:var(--ink);font-family:'Geist',sans-serif;font-size:13px;width:100%;resize:none;min-height:22px;max-height:120px;line-height:1.5;font-weight:400}
.ci::placeholder{color:var(--ink-faint)}
.cr{display:flex;align-items:center;justify-content:space-between}
.cto{display:flex;gap:2px}
.ctool{background:none;border:none;color:var(--ink-faint);width:26px;height:26px;border-radius:5px;cursor:pointer;font-size:13px;transition:all .15s}
.ctool:hover{color:var(--ink);background:rgba(255,255,255,.04)}
.send{padding:6px 14px;border-radius:6px;background:var(--coral);color:#1a1b1e;border:none;cursor:pointer;font-family:'Geist',sans-serif;font-size:11px;font-weight:500;transition:all .15s}
.send:hover{background:var(--coral-soft)}
.demo{position:fixed;bottom:40px;left:40px;display:flex;gap:3px;align-items:center;background:var(--surface-2);border:1px solid var(--line-2);padding:5px 6px 5px 12px;border-radius:8px;z-index:30;box-shadow:0 10px 30px rgba(0,0,0,.4)}
.dl{font-family:'Geist',sans-serif;font-size:10px;color:var(--ink-faint);margin-right:4px}
.demo button{background:transparent;border:1px solid transparent;color:var(--ink-dim);font-family:'Geist',sans-serif;font-size:11px;padding:4px 10px;border-radius:5px;cursor:pointer;transition:all .15s}
.demo button:hover{color:var(--ink)}
.demo button.active{color:var(--coral);border-color:rgba(232,120,88,.3);background:rgba(232,120,88,.08)}
</style></head>
<body><div class="app">
<div class="tl-dots"><span></span><span></span><span></span></div>
<div class="marquee">
<div class="brand-mark">Roy</div>
<div class="session-strip">
<div class="strip-item"><span class="strip-k">Agent</span><span class="strip-v">claude-opus-4</span></div>
<div class="strip-item"><span class="strip-k">Workspace</span><span class="strip-v">ishoo / ISS-04</span></div>
<div class="strip-item"><span class="strip-k">Sandbox</span><span class="strip-v">active</span></div>
<div class="strip-item"><span class="strip-k">Elapsed</span><span class="strip-v">08:42</span></div>
</div>
<div class="session-health status-chip" id="statusChip"><div class="heart"></div><div class="health-text" id="statusText">Running normally</div></div>
</div>
<div class="main"><div class="canvas"><div class="pod">
<div class="pod-frame">
<div class="rivet tl"></div><div class="rivet tr"></div><div class="rivet bl"></div><div class="rivet br"></div>
<div class="terminal">
<div class="terminal-head"><div class="tdot"></div><div class="tdot"></div><div class="tdot"></div><div class="term-label">session · <em>ishoo / ISS-04</em></div></div>
<div class="tb" id="terminal">
<span class="line"><span class="dim">── sandbox initialized · policy strict · network blocked ──</span></span>
<span class="line">&nbsp;</span>
<span class="line"><span class="mint">▸</span> inspecting workspace</span>
<span class="line"><span class="dim">  247 files · rust · dioxus · git clean</span></span>
<span class="line"><span class="mint">▸</span> reading <span class="cream">src/core/issue.rs</span></span>
<span class="line"><span class="mint">▸</span> reading <span class="cream">src/store/mod.rs</span></span>
<span class="line"><span class="mint">▸</span> planning changes for ISS-04</span>
<span class="line"><span class="dim">  ↳ add atomic write path for issue mutations</span></span>
<span class="line"><span class="dim">  ↳ gate on content hash match</span></span>
<span class="line">&nbsp;</span>
<span class="line"><span class="prompt">$</span> cargo check</span>
<span class="line"><span class="mint">    Finished</span> <span class="dim">dev in 2.14s</span></span>
<span class="line">&nbsp;</span>
<span class="line"><span class="mint">▸</span> editing <span class="cream">src/store/atomic.rs</span> <span class="peach">+87 −12</span></span>
<span class="line"><span class="mint">▸</span> editing <span class="cream">src/store/mod.rs</span> <span class="peach">+14 −3</span></span>
<span class="line"><span class="prompt">$</span> cargo clippy -- -D warnings</span>
<span class="line"><span class="mint">    clippy clean · 0 warnings</span></span>
<span class="line">&nbsp;</span>
<span class="line"><span class="prompt">$</span> cargo test store::atomic</span>
<span class="line"><span class="mint">test store::atomic::write_succeeds_on_match ... ok</span></span>
<span class="line"><span class="mint">test store::atomic::write_rejects_on_conflict ... ok</span></span>
<span class="line"><span class="mint">test store::atomic::concurrent_writes_serialize ... ok</span></span>
<span class="line" id="streamTarget"></span>
<span class="line"><span class="prompt">$</span><span class="cursor"></span></span>
</div></div></div>
<div class="edge">
<button class="eb" data-drawer="attention"><span>!</span><span class="badge"></span><span class="tip">Attention</span></button>
<button class="eb" data-drawer="review"><span>R</span><span class="tip">Review</span></button>
<button class="eb" data-drawer="activity"><span>A</span><span class="tip">Activity</span></button>
<button class="eb" data-drawer="diag"><span>D</span><span class="tip">Diagnostics</span></button>
</div>
</div></div>
<div class="drawer" id="drawer-attention"><div class="dh"><div><div class="ds">— Needs you —</div><div class="dt">Attention</div></div><button class="dc" data-close>×</button></div><div class="db">
<div class="card urgent"><div class="cl">Blocked · Network</div><div class="ct">Reach crates.io?</div><div class="cb">Sandbox policy requires approval for outbound network. Agent needs this to fetch dependencies.</div><div class="ca"><button class="btn primary">Allow once</button><button class="btn">Allow session</button><button class="btn ghost">Deny</button></div></div>
<div class="card"><div class="cl">Approval · Overwrite</div><div class="ct">Overwrite src/store/mod.rs?</div><div class="cb">File has uncommitted changes. Agent wants to replace contents.</div><div class="ca"><button class="btn primary">Approve</button><button class="btn">View diff</button><button class="btn ghost">Reject</button></div></div>
<div class="card"><div class="cl">Progress · Idle 94s</div><div class="ct">Agent may be stuck</div><div class="cb">Last action: reading <code>Cargo.lock</code>. Consider nudging with guidance.</div><div class="ca"><button class="btn">Nudge</button><button class="btn ghost">Interrupt</button></div></div>
</div></div>
<div class="drawer" id="drawer-review"><div class="dh"><div><div class="ds">— Outcome —</div><div class="dt">Review</div></div><button class="dc" data-close>×</button></div><div class="db">
<div class="card"><div class="cl" style="color:var(--mint)">✓ Validation passed</div><div class="ct">Atomic writes, gated on hash</div><div class="cb">Added <code>write_if_unchanged</code> in <code>src/store/atomic.rs</code>. Wired through <code>store::put</code>. All 6 tests pass. Clippy clean.</div></div>
<div class="ds" style="margin:18px 0 8px">— Changed files —</div>
<div class="fl"><span class="fn">src/store/atomic.rs</span><span class="fm">+87 −12</span></div>
<div class="fl"><span class="fn">src/store/mod.rs</span><span class="fm">+14 −3</span></div>
<div class="fl"><span class="fn">tests/atomic_test.rs</span><span class="fm">+42</span></div>
<div class="ca" style="margin-top:20px"><button class="btn primary">Approve & merge</button><button class="btn">Request revision</button><button class="btn ghost">Reject</button></div>
</div></div>
<div class="drawer" id="drawer-activity"><div class="dh"><div><div class="ds">— Session —</div><div class="dt">Activity</div></div><button class="dc" data-close>×</button></div><div class="db">
<div class="ev"><span class="et">14:02</span><span class="ed"><strong>Review ready</strong> — ISS-04 atomic writes</span></div>
<div class="ev"><span class="et">14:01</span><span class="ed">Validation passed · 6 tests ok</span></div>
<div class="ev"><span class="et">14:00</span><span class="ed">Running <strong>cargo test</strong></span></div>
<div class="ev"><span class="et">13:58</span><span class="ed">Edited <strong>src/store/atomic.rs</strong></span></div>
<div class="ev"><span class="et">13:57</span><span class="ed">Blocked action recovered · crates.io denied</span></div>
<div class="ev"><span class="et">13:55</span><span class="ed">Reading <strong>src/store/mod.rs</strong></span></div>
<div class="ev"><span class="et">13:54</span><span class="ed">Inspecting workspace · 247 files</span></div>
<div class="ev"><span class="et">13:54</span><span class="ed">Session began</span></div>
</div></div>
<div class="drawer" id="drawer-diag"><div class="dh"><div><div class="ds">— Internals —</div><div class="dt">Diagnostics</div></div><button class="dc" data-close>×</button></div><div class="db">
<div class="ds" style="margin-bottom:10px">— Session —</div>
<div class="diag"><span class="dk">session_id</span><span class="dv">roy_0198_4af2</span></div>
<div class="diag"><span class="dk">backend</span><span class="dv">local/nsjail</span></div>
<div class="diag"><span class="dk">policy</span><span class="dv">strict.v3</span></div>
<div class="diag"><span class="dk">uptime</span><span class="dv">00:08:42</span></div>
<div class="ds" style="margin:20px 0 10px">— Events —</div>
<div class="diag"><span class="dk">commands_run</span><span class="dv">14</span></div>
<div class="diag"><span class="dk">files_read</span><span class="dv">31</span></div>
<div class="diag"><span class="dk">files_written</span><span class="dv">3</span></div>
<div class="diag"><span class="dk">policy_denials</span><span class="dv">1</span></div>
</div></div>
</div>
<div class="cw"><div class="composer">
<textarea class="ci" id="composer" placeholder="Guide the agent…" rows="1"></textarea>
<div class="cr"><div class="cto"><button class="ctool">+</button><button class="ctool">◫</button><button class="ctool">@</button></div><button class="send">Send ⏎</button></div>
</div></div>
<div class="demo"><span class="dl">State</span>
<button data-state="normal" class="active">Normal</button>
<button data-state="attention">Attention</button>
<button data-state="review">Review</button>
<button data-state="failed">Failed</button>
<button data-state="exited">Exited</button>
</div>
</div>
<script>
const tabs=document.querySelectorAll('.eb'),drawers=document.querySelectorAll('.drawer');
function closeAll(){drawers.forEach(d=>d.classList.remove('open'));tabs.forEach(t=>t.classList.remove('active'))}
tabs.forEach(tab=>tab.addEventListener('click',()=>{const n=tab.dataset.drawer,d=document.getElementById('drawer-'+n),w=d.classList.contains('open');closeAll();if(!w){d.classList.add('open');tab.classList.add('active')}}));
document.querySelectorAll('[data-close]').forEach(b=>b.addEventListener('click',closeAll));
const chip=document.getElementById('statusChip'),statusText=document.getElementById('statusText'),attBtn=document.querySelector('[data-drawer="attention"]');
const states={normal:{cls:'',text:'Running normally',alert:false},attention:{cls:'needs-attention',text:'Needs attention',alert:true},review:{cls:'review',text:'Review ready',alert:false},failed:{cls:'failed',text:'Validation failed',alert:true},exited:{cls:'exited',text:'Session ended',alert:false}};
document.querySelectorAll('.demo button').forEach(btn=>btn.addEventListener('click',()=>{
  document.querySelectorAll('.demo button').forEach(b=>b.classList.remove('active'));btn.classList.add('active');
  const s=states[btn.dataset.state];chip.className='session-health status-chip '+s.cls;statusText.textContent=s.text;attBtn.classList.toggle('has-alert',s.alert);
  if(btn.dataset.state==='review'){closeAll();document.getElementById('drawer-review').classList.add('open');document.querySelector('[data-drawer="review"]').classList.add('active')}
  else if(btn.dataset.state==='attention'||btn.dataset.state==='failed'){closeAll();document.getElementById('drawer-attention').classList.add('open');attBtn.classList.add('active')}
}));
const st=document.getElementById('streamTarget'),term=document.getElementById('terminal');
const streams=['<span class="mint">test store::atomic::rejects_stale_hash ... ok</span>','<span class="mint">test store::atomic::preserves_on_failure ... ok</span>','<span class="mint">test store::atomic::fsync_on_commit ... ok</span>','<span class="dim">&nbsp;</span>','<span class="mint">test result: ok. 6 passed; 0 failed</span>','<span class="dim">&nbsp;</span>','<span class="mint">▸</span> <span class="cream">all checks green · preparing review</span>'];
let si=0;function stream(){if(si<streams.length){const s=document.createElement('span');s.className='line';s.innerHTML=streams[si];st.parentNode.insertBefore(s,st.nextSibling);term.scrollTop=term.scrollHeight;si++;setTimeout(stream,900+Math.random()*500)}}
setTimeout(stream,1800);
const c=document.getElementById('composer');c.addEventListener('input',()=>{c.style.height='auto';c.style.height=Math.min(c.scrollHeight,120)+'px'});
document.addEventListener('keydown',e=>{if(e.key==='Escape')closeAll()});
</script></body></html>

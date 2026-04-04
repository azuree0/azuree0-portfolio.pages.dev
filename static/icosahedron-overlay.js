/**
 * 3D icosahedron for the first repo card overlay (hero tagline hover).
 * Depends on global THREE (three.js r128, loaded before this script).
 */
(function () {
  var state = {
    raf: null,
    renderer: null,
    scene: null,
    camera: null,
    mesh: null,
    host: null,
    ro: null,
    onWinResize: null,
  };

  function dispose() {
    if (state.raf) {
      cancelAnimationFrame(state.raf);
      state.raf = null;
    }
    if (state.onWinResize) {
      window.removeEventListener("resize", state.onWinResize);
      state.onWinResize = null;
    }
    if (state.ro && state.host) {
      try {
        state.ro.disconnect();
      } catch (e) {}
      state.ro = null;
    }
    if (state.renderer) {
      try {
        state.renderer.dispose();
      } catch (e) {}
      if (state.host && state.renderer.domElement && state.renderer.domElement.parentNode === state.host) {
        state.host.removeChild(state.renderer.domElement);
      }
    }
    state.renderer = null;
    state.scene = null;
    state.camera = null;
    state.mesh = null;
    state.host = null;
  }

  function resize() {
    if (!state.renderer || !state.camera || !state.host) return;
    var w = state.host.clientWidth;
    var h = state.host.clientHeight;
    if (w < 2 || h < 2) return;
    state.camera.aspect = w / h;
    state.camera.updateProjectionMatrix();
    state.renderer.setSize(w, h);
  }

  function init() {
    dispose();
    var host = document.getElementById("first-repo-icosahedron-host");
    if (!host || typeof THREE === "undefined") return;
    state.host = host;
    var w = Math.max(host.clientWidth, 280);
    var h = Math.max(host.clientHeight, 160);

    var scene = new THREE.Scene();
    state.scene = scene;
    var cam = new THREE.PerspectiveCamera(42, w / h, 0.1, 100);
    cam.position.set(0, 0, 3.6);
    state.camera = cam;

    var geo = new THREE.IcosahedronGeometry(1.15, 0);
    var mat = new THREE.MeshPhongMaterial({
      color: 0x0eacc7,
      emissive: 0x061a22,
      shininess: 95,
      specular: 0x9ce4f2,
      flatShading: true,
    });
    var mesh = new THREE.Mesh(geo, mat);
    scene.add(mesh);
    state.mesh = mesh;

    scene.add(new THREE.AmbientLight(0x6dd4e3, 0.45));
    var dl = new THREE.DirectionalLight(0xffffff, 0.85);
    dl.position.set(2.2, 3.5, 4);
    scene.add(dl);
    var fill = new THREE.DirectionalLight(0x1d395e, 0.35);
    fill.position.set(-3, -1, -2);
    scene.add(fill);

    var r = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    r.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    r.setSize(w, h);
    r.setClearColor(0x000000, 0);
    host.appendChild(r.domElement);
    state.renderer = r;

    state.onWinResize = function () {
      resize();
    };
    window.addEventListener("resize", state.onWinResize);

    if (typeof ResizeObserver !== "undefined") {
      state.ro = new ResizeObserver(function () {
        resize();
      });
      state.ro.observe(host);
    }

    var t0 = performance.now();
    function loop(t) {
      if (!state.mesh || !state.renderer) return;
      var dt = (t - t0) * 0.001;
      state.mesh.rotation.x = dt * 0.55;
      state.mesh.rotation.y = dt * 0.72;
      r.render(scene, cam);
      state.raf = requestAnimationFrame(loop);
    }
    state.raf = requestAnimationFrame(loop);
  }

  function waitThreeThenInit() {
    if (typeof THREE !== "undefined") {
      init();
      return;
    }
    var n = 0;
    function retry() {
      if (typeof THREE !== "undefined") {
        init();
        return;
      }
      if (++n > 250) return;
      setTimeout(retry, 16);
    }
    retry();
  }

  window.portfolioInitIcosahedron = waitThreeThenInit;
  window.portfolioDisposeIcosahedron = dispose;
})();

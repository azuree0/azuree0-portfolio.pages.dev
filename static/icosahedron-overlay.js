/**
 * Rotating icosahedron in .icosahedron-overlay-host (triggered by hero tagline hover).
 */
(function () {
  var SELECTOR = ".icosahedron-overlay-host";
  var ATTR = "data-ico-ready";

  function dispose(container) {
    var st = container._icoState;
    if (!st) return;
    if (st.raf) cancelAnimationFrame(st.raf);
    if (st.ro) st.ro.disconnect();
    try {
      if (st.renderer) st.renderer.dispose();
      if (st.mesh) {
        if (st.mesh.geometry) st.mesh.geometry.dispose();
        if (st.mesh.material) st.mesh.material.dispose();
      }
    } catch (e) {}
    if (st.canvas && st.canvas.parentNode) st.canvas.parentNode.removeChild(st.canvas);
    container.removeAttribute(ATTR);
    container._icoState = null;
  }

  function initHost(container) {
    if (!container || container.getAttribute(ATTR)) return;
    if (typeof THREE === "undefined") return;

    var w = Math.max(container.clientWidth, 200);
    var h = Math.max(container.clientHeight, 120);

    var scene = new THREE.Scene();
    var camera = new THREE.PerspectiveCamera(42, w / h, 0.1, 100);
    /* Pull camera back so the scaled mesh reads a bit smaller in frame */
    camera.position.z = 2.5;

    var geo = new THREE.IcosahedronGeometry(1, 0);
    /* Silver: high key vs shadow contrast (low ambient/emissive, strong specular) */
    var mat = new THREE.MeshPhongMaterial({
      color: 0xa8b0bc,
      specular: 0xffffff,
      emissive: 0x050608,
      shininess: 100,
      flatShading: true,
      transparent: true,
      opacity: 0.98,
    });
    var mesh = new THREE.Mesh(geo, mat);
    mesh.scale.set(0.82, 0.82, 0.82);
    scene.add(mesh);

    scene.add(new THREE.AmbientLight(0xc8ccd8, 0.16));
    var dl = new THREE.DirectionalLight(0xffffff, 1.35);
    dl.position.set(2.8, 4.2, 3.2);
    scene.add(dl);
    var glint = new THREE.DirectionalLight(0xe8eeff, 0.32);
    glint.position.set(-1.2, 2.5, 4);
    scene.add(glint);
    var back = new THREE.DirectionalLight(0x607080, 0.1);
    back.position.set(-2.2, -1.2, -2);
    scene.add(back);

    var renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    renderer.setSize(w, h);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    renderer.domElement.style.display = "block";
    renderer.domElement.style.width = "100%";
    renderer.domElement.style.height = "100%";
    container.appendChild(renderer.domElement);
    container.setAttribute(ATTR, "1");

    var st = {
      renderer: renderer,
      mesh: mesh,
      canvas: renderer.domElement,
      raf: null,
      ro: null,
    };
    container._icoState = st;

    function tick() {
      if (!document.body.contains(container)) {
        dispose(container);
        return;
      }
      mesh.rotation.x += 0.0025;
      mesh.rotation.y += 0.0038;
      renderer.render(scene, camera);
      st.raf = requestAnimationFrame(tick);
    }
    tick();

    st.ro = new ResizeObserver(function () {
      if (!document.body.contains(container)) return;
      var nw = Math.max(container.clientWidth, 200);
      var nh = Math.max(container.clientHeight, 120);
      camera.aspect = nw / nh;
      camera.updateProjectionMatrix();
      renderer.setSize(nw, nh);
    });
    st.ro.observe(container);
  }

  function scan() {
    var nodes = document.querySelectorAll(SELECTOR + ":not([" + ATTR + "])");
    for (var i = 0; i < nodes.length; i++) initHost(nodes[i]);
  }

  function waitThree() {
    if (typeof THREE !== "undefined") {
      scan();
      new MutationObserver(scan).observe(document.body, { childList: true, subtree: true });
      return;
    }
    setTimeout(waitThree, 30);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", waitThree);
  } else {
    waitThree();
  }
})();

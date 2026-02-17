/**
 * Light caustics at top - procedural underwater caustics using Three.js
 * Waits for #caustics-container to appear (mounted by Yew), then initializes.
 */
(function () {
  if (typeof THREE === "undefined") return;

  function initCaustics() {
    const container = document.getElementById("caustics-container");
    if (!container) return;

    const width = container.clientWidth;
    const height = container.clientHeight;
    if (!width || !height) return;

    const scene = new THREE.Scene();
    const camera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0, 1);

    const causticsVertexShader = `
      varying vec2 vUv;
      void main() {
        vUv = uv;
        gl_Position = vec4(position, 1.0);
      }
    `;

    const causticsFragmentShader = `
      uniform float uTime;
      uniform vec2 uResolution;
      varying vec2 vUv;

      // Procedural caustics - sharper sine waves (higher freq, steeper pow)
      float caustic(vec2 p) {
        float t = uTime * 0.4;
        float c = 0.0;
        c += sin(p.x * 22.0 + t) * sin(p.y * 22.0 + t * 1.3);
        c += sin(p.x * 28.0 - t * 0.7) * sin(p.y * 28.0 + t * 1.1);
        c += sin((p.x + p.y) * 14.0 + t * 0.5);
        c = c * 0.33 + 0.5;
        return pow(c, 5.0);
      }

      void main() {
        vec2 uv = vUv;
        float c = caustic(uv);
        // Fade toward bottom - caustics stronger at top (surface)
        float topFade = 1.0 - uv.y;
        topFade = pow(topFade, 1.2);
        c *= topFade;
        // Cyan/white light to match underwater aesthetic
        vec3 color = vec3(0.4, 0.85, 0.95);
        float alpha = c * 0.22 * topFade;
        gl_FragColor = vec4(color, alpha);
      }
    `;

    const material = new THREE.ShaderMaterial({
      vertexShader: causticsVertexShader,
      fragmentShader: causticsFragmentShader,
      uniforms: {
        uTime: { value: 0 },
        uResolution: { value: new THREE.Vector2(width, height) },
      },
      transparent: true,
      depthWrite: false,
      blending: THREE.AdditiveBlending,
    });

    const geometry = new THREE.PlaneGeometry(2, 2);
    const mesh = new THREE.Mesh(geometry, material);
    scene.add(mesh);

    const renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio || 1);
    renderer.clearColor(0, 0, 0, 0);
    container.appendChild(renderer.domElement);

    function animate() {
      material.uniforms.uTime.value = performance.now() * 0.001;
      renderer.render(scene, camera);
      requestAnimationFrame(animate);
    }
    animate();

    function onResize() {
      const w = container.clientWidth;
      const h = container.clientHeight;
      if (w && h) {
        renderer.setSize(w, h);
        material.uniforms.uResolution.value.set(w, h);
      }
    }
    window.addEventListener("resize", onResize);
  }

  if (document.getElementById("caustics-container")) {
    initCaustics();
  } else {
    const observer = new MutationObserver(function () {
      if (document.getElementById("caustics-container")) {
        observer.disconnect();
        initCaustics();
      }
    });
    observer.observe(document.body, { childList: true, subtree: true });
  }
})();

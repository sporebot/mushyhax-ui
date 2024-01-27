<script>
  import Greet from './lib/Greet.svelte'
</script>

<main class="container">
  <div style="display: flex; margin-left: auto; margin-right: auto; min-width: 210px">
    <h1 style="margin-bottom: 50px;">Login to</h1>
    <img src="/mshx.svg" class="login-logo"/>
  </div>

  <input placeholder="Username" class="login-box" id="userbox">
  <input placeholder="Password" type="password" class="login-box" id="passbox">

  <p style="margin: 5px; font-weight: 600;">insert random fact here</p>
  <button id="login-button" class="login-button button">Login</button>

  <!-- <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div> -->

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>

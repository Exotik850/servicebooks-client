<script>
  // @ts-nocheck

  export let prev;
  export let next;
  export let step;
  export let maxStep = 3;
</script>

<div class="form-navigation">
  <button class="nav" on:click|preventDefault={prev} disabled={step === 0}
    ><i class="material-icons">arrow_back</i></button
  >
  <button class="nav" on:click|preventDefault={next} disabled={step === maxStep}
    ><i class="material-icons">arrow_forward</i></button
  >
</div>

<style>
  .form-navigation {
    display: flex;
    position: fixed;
    gap: 20px;
    bottom: 10px;
    margin: 0 auto;
    width: 95vw;
    justify-content: space-evenly;
    pointer-events: none;
  }
  .nav {
    background: none;
    font-size: 2rem;
    color: white;
    opacity: 0.5;
    transition: 0.3s ease;
    width: 40px;
    height: 40px;
    pointer-events: all;
    text-align: left;
    border-radius: 50%;
    line-height: 0px;
  }
  .nav:hover {
    opacity: 1;
    transform: scale(1.1);
  }
  .nav:active {
    transform: scale(0.9);
  }
  .nav:focus,
  .nav:active,
  .nav {
    outline-width: 0;
    outline: none;
    border: none;
    box-shadow: none;
  }
</style>

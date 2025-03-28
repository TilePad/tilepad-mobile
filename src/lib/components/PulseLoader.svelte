<div class="loader"></div>

<style>
  .loader {
    width: 64px;
    height: 64px;
    display: inline-block;
    position: relative;
  }
  .loader::after,
  .loader::before {
    content: "";
    box-sizing: border-box;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    border: 2px solid #fff;
    position: absolute;
    left: 0;
    top: 0;
    transform: scale(0);
    opacity: 1;
    animation: pulse 2s linear infinite;
  }
  .loader::after {
    animation-delay: 1s;
  }

  @keyframes pulse {
    0% {
      transform: scale(0);
      opacity: 1;
    }
    100% {
      transform: scale(1);
      opacity: 0;
    }
  }
</style>

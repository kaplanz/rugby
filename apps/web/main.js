import { mount } from "svelte";
import App from "/www/App.svelte";

export default mount(App, {
  target: document.querySelector("body"),
});

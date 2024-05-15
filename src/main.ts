import "virtual:uno.css";
import "primeicons/primeicons.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";

import PrimeVue from "primevue/config";
import Ripple from "primevue/ripple";
import ToastService from "primevue/toastservice";
import Button from "primevue/button";
import Stepper from "primevue/stepper";
import StepperPanel from "primevue/stepperpanel";
import IconField from "primevue/iconfield";
import InputIcon from "primevue/inputicon";
import Toast from "primevue/toast";

const app = createApp(App);
const pinia = createPinia();

app.use(PrimeVue, {
  ripple: true,
  theme: {
    options: {
      prefix: "p",
      darkModeSelector: ".dark",
      cssLayer: false,
    },
  },
});
app.directive("ripple", Ripple);
app.component("Toast", Toast);
app.component("Button", Button);
app.component("Stepper", Stepper);
app.component("StepperPanel", StepperPanel);
app.component("IconField", IconField);
app.component("InputIcon", InputIcon);
app.use(ToastService);

app.use(pinia);
app.use(router);

app.mount("#app");

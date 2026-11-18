import DefaultTheme from 'vitepress/theme';
import { onMounted } from 'vue';
import type { Theme } from 'vitepress';
import './style.css';

let hydrated = false;
export default {
  extends: DefaultTheme,
  enhanceApp({ router }) {
    if (typeof window === 'undefined') return;
    const initial = new URL(location.href);
    router.onBeforeRouteChange = (href) => {
      const next = new URL(href, location.href);
      // The first lean page module hydrates server HTML. Changing the route
      // before mount would hydrate the destination against the previous page.
      if (!hydrated && (next.pathname !== initial.pathname || next.search !== initial.search)) {
        location.assign(next.href);
        return false;
      }
    };
  },
  setup() {
    onMounted(() => { hydrated = true; });
  },
} satisfies Theme;

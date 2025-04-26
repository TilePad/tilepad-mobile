import { runeStore } from "$lib/utils/svelte.svelte";
import { createQuery } from "@tanstack/svelte-query";

export function useFontLoader(
  options: () => {
    serverURL: string;
    fontName: string;
    bold: boolean;
    italic: boolean;
  },
) {
  return createQuery(
    runeStore(() => {
      const { serverURL, fontName, bold, italic } = options();
      return {
        // We don't need to load roboto as its always available since
        // we provide it out of the box
        enabled: fontName !== "Roboto",
        queryKey: ["font", fontName, `${italic}`, `${bold}`],
        queryFn: async () => {
          const params = new URLSearchParams();
          params.append("bold", `${bold}`);
          params.append("italic", `${italic}`);
          const font = new FontFace(
            fontName,
            `url(${serverURL}/fonts/${encodeURIComponent(fontName)}?${params.toString()})`,
            {
              style: italic ? "italic" : "normal",
              weight: bold ? "700" : "400",
            },
          );
          const loaded = await font.load();
          document.fonts.add(loaded);
          return true;
        },
        staleTime: Infinity,
      };
    }),
  );
}

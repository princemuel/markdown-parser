import { parse } from "markdown-parser";
import { getElement } from "./dom";

const button = getElement("#parse", HTMLButtonElement);
const previewArea = getElement("#output", HTMLElement);

button.addEventListener("click", parse_markdown);

function parse_markdown() {
  previewArea.innerHTML = parse(
    getElement("#markdown", HTMLTextAreaElement).value
  );
}

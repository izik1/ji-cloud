import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import {MODE} from "@elements/module/memory/_common/types.ts";

const STR_HEADER = "Select theme";
@customElement('step2-sidebar-container')
export class _ extends LitElement {
  static get styles() {
      return [css`
          .options {
              margin-top: 24px;
              display: grid;
              grid-template-columns: repeat(2, 1fr);
          }
    `];
  }

  render() {

      return html`
          <header>${STR_HEADER}</header>
          <div class="options">
              <slot></slot>
          </div>
      `
  }
}
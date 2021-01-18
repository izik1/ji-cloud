import { LitElement, html, css, customElement, property } from 'lit-element';
import { classMap } from 'lit-html/directives/class-map';
import {nothing} from "lit-html";
@customElement('dropdown-select')
export class _ extends LitElement {

  static get styles() {
    return [css`
   .input-wrapper{
    position:relative;
    width:inherit;
    height:64px;
    border: solid 1px #89b3ff;
    border-radius:14px;
    padding: 8px 48px 8px 16px;
   }
   input{
    outline:none;
    border:none;
    margin-top: 33px;
    width: inherit;
   }
   label{
    position:absolute;
    top: 0;
    left: 0;
    font-size: 16px;
    padding: 8px 0px 0px 16px;
    color: #5590fc;
   }
   .input-wrapper:active{
    border: solid 2px #5590fc;
    margin: -1px;
   }
   input{ font-size:16px;}
   img-ui{
    position: absolute;
    top: 33%;
    right: 20px;
    transform: rotate(180deg)
   }
   .errorwrapper{
    border: solid 1px #f00813;
    background-color: #fff4f4;
   }
   .errorwrapper input{
    background-color: #fff4f4;
   }
   .error{
    font-size: 14px;
    color: #f00813;
    margin-top:4px;
    font-weight: 500;
    padding-left:8px;
    display:block;
   }
   
   
   .open {
     display:block;
     box-shadow: 0 3px 6px 0 rgba(0, 0, 0, 0.16);
     width:100%;
     position:absolute;
     left:0;
     top:80px;
     border-radius:0 0 14px 14px;
     z-index:-1;
   }
   ::slotted([slot=inner-dropdown]){
      padding:16px;
      display:none;
   }
   .open ::slotted([slot=inner-dropdown]){
     display:block
   }
   
  
    `];
  }

  @property()
  label: string = "";

  @property()
  errormessage: string = "";

  @property({type:Boolean})
  instruction: boolean = false;

  @property()
  helpertext: string = "";

  @property()
  error: string = "";


  @property({type:Boolean})
  errorwrapper: boolean = false;

  @property({type:Boolean})
  open: boolean = false;

  render() {

    const {label, open, error} = this;
    const isError:boolean = error !== "";

    const errorwrapper = isError ? "errorwrapper" : "";

    return html`
    
    <div class="input-wrapper ${errorwrapper ? "errorwrapper" : ''}">
        <input placeholder="Placeholder" type="text" class="">
        <label class="">${label}</label>
        <img-ui path="icn-chevron-dropdown-up.svg"></img-ui>
        <div class="${open ? "open" : ''}">
        
          <slot name="inner-dropdown"></slot>
        </div>
    </div>

     ${
      isError 
        ? html`<p class="error">${error}</p>` 
        : nothing
    }
  `;
  }
}
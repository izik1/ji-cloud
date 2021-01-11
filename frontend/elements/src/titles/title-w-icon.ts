import { MEDIA_UI } from '@utils/path';
import { LitElement, html, css, customElement, property } from 'lit-element';
@customElement('title-wicon')
export class _ extends LitElement {
  static get styles() {
    return [css`
        .wrapper{
            display: flex;
            align-items:center;
        }
        .uploaded{
            color: #46ba6f;
        }
        p{
            font-size: 13px;
            font-weight: 500;
        }
        img-ui{
          margin-right: 12px
        }
        
   
    `];
  }

  @property()
  title:string = ""; 
  @property()
  path:string = ""; 
  @property({type: Boolean})
  uploaded: boolean = false;

  render() {

    const {title, path, uploaded} = this;

    return html`
    <div class="wrapper">
        <img-ui path="${path}"></img-ui>
        <p class="${uploaded}">${title}</p>
        
    </div>
  `;
  }
}
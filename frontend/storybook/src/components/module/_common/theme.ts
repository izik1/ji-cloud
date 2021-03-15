import {ThemeKind} from "@elements/module/_common/theme";
export * from "@elements/module/_common/theme";

export const ThemeKinds:Array<ThemeKind> = ["", "chalkboard", "happy-brush"];

export const ThemeControl = {
    control: {
            type: 'inline-radio',
            options: ThemeKinds 
    }
}

import "@elements/module/_common/post-preview/post-preview-action";
import { argsToAttrs } from "@utils/attributes";
import { Kind } from "@elements/module/_common/post-preview/post-preview-action";

export default {
    title: "Module / _common / Post Preview"
}

interface Args {
    kind: Kind,
}

const DEFAULT_ARGS:Args = {
    kind: "flashcards",
}

export const PostPreviewAction = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <post-preview-action ${argsToAttrs(props)}></post-preview-action>
    `;
}

PostPreviewAction.args = DEFAULT_ARGS;
PostPreviewAction.argTypes = {
    kind: {
        control: {
            type: 'inline-radio',
            options: ['1of3', 'matching', 'flashcards', 'print', 'continue']
        }
    }
}
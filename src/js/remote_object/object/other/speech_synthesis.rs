use super::*;

js_remote_object!{
    /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis>
    class SpeechSynthesis extends Object {
        static #type: "object";
        static #subtype: "other";
        static #class: "SpeechSynthesis";

        properties: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/paused>
            paused: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pending>
            pending: bool [readonly];

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speaking>
            speaking: bool [readonly];
        }

        methods: {
            /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/cancel>
            cancel() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/getVoices>
            getVoices() -> Vec<JsObject>;

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pause>
            pause() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/resume>
            resume() -> ();

            /// <https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speak>
            speak(utterance: impl IntoJs<JsObject>) -> ();
        }
    }
}
 
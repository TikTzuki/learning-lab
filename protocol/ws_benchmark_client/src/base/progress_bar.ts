import _progress from 'cli-progress';
import {ProgressObj} from "./dto";

class ProgressBar {
    progress_obj: ProgressObj & { bar?: any; timer?: NodeJS.Timeout };

    constructor(progress_obj: ProgressObj & { bar?: any; timer?: NodeJS.Timeout }) {
        this.progress_obj = progress_obj;
    }

    clear(): void {
        this.progress_obj.counter = 0;
    }

    start(): void {
        const self = this;
        console.log(this.progress_obj.message);
        this.progress_obj.bar = new _progress.Bar({}, _progress.Presets.shades_grey);
        this.progress_obj.bar.start(this.progress_obj.total, 0);
        this.progress_obj.timer = setInterval(function () {
            self.progress_obj.bar.update(self.progress_obj.counter);
            if (self.progress_obj.counter >= self.progress_obj.bar.getTotal()) {
                clearInterval(self.progress_obj.timer);
                self.progress_obj.bar.stop();
            }
        }, 50);
    }

    stop(): void {
        clearInterval(this.progress_obj.timer);
        this.progress_obj.bar.stop();
    }
}

export default ProgressBar;

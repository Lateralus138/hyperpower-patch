fn main() {
    #[path = "utils/functions.rs"] mod functions;
    use colored::Colorize;
    use functions::var_os_or_exit;
    use std::{path::Path,process::exit,fs::File,io::Write};
    let home = var_os_or_exit("HOME",1);
    let mut index_file = String::new();
    index_file.push_str(&home.to_string_lossy());
    index_file.push_str("/.hyper_plugins/node_modules/hyperpower/index.js");
    let path = Path::new(&index_file);
    let path = match path.is_file() {
        true => {
            let string: &str = "exists.";
            println!(" {} {}"
                ,path
                    .to_string_lossy()
                    .blue()
                    .bold(),
                string
                    .green()
                    .bold()
                );
            path.to_string_lossy()
        }
        false => {
            let string: &str = "does not exist.";
            println!(" {} {}",
                path
                    .to_string_lossy()
                    .blue().bold(),
                string
                    .red().bold()
                );
            exit(1);
        }
    };
    const FILE_TEXT: &str = "const throttle = require('lodash.throttle');\n\
    const Color = require('color');\n\
    const nameToHex = require('convert-css-color-name-to-hex');\n\
    const toHex = (str) => Color(nameToHex(str)).hexString();\n\
    const values = require('lodash.values');\n\n\
    // Constants for the particle simulation.\n\
    const MAX_PARTICLES = 500;\n\
    const PARTICLE_NUM_RANGE = () => 5 + Math.round(Math.random() * 5);\n\
    const PARTICLE_GRAVITY = 0.075;\n\
    const PARTICLE_ALPHA_FADEOUT = 0.96;\n\
    const PARTICLE_VELOCITY_RANGE = {\n\t\
        x: [-1, 1],\n\t\
        y: [-3.5, -1.5]\n\
    };\n\
    const PARTICLE_ALPHA_MIN_THRESHOLD = 0.1;\n\n\
    // Our extension's custom redux middleware. Here we can intercept redux actions and respond to them.\n\
    exports.middleware = (store) => (next) => (action) => {\n\t\
        // the redux `action` object contains a loose `type` string, the\n\t\
        // 'SESSION_ADD_DATA' type identifier corresponds to an action in which\n\t\
        // the terminal wants to output information to the GUI.\n\t\
        if ('SESSION_ADD_DATA' === action.type) {\n\n\t\t
            // 'SESSION_ADD_DATA' actions hold the output text data in the `data` key.\n\t\t\
            const { data } = action;\n\t\t\
            if (detectWowCommand(data)) {\n\t\t\t\
                // Here, we are responding to 'wow' being input at the prompt. Since we don't\n\t\t\t\
                // want the \"unknown command\" output being displayed to the user, we don't thunk the next\n\t\t\t\
                // middleware by calling `next(action)`. Instead, we dispatch a new action 'WOW_MODE_TOGGLE'.\n\t\t\t\
                store.dispatch({\n\t\t\t\t\
                    type: 'WOW_MODE_TOGGLE'\n\t\t\t\
                });\n\t\t\
            } else {\n\t\t\t\
                next(action);\n\t\t\
            }\n\t\
        } else {\n\t\t\
            next(action);\n\t\
        }\n\
    };\n\n\
    // This function performs regex matching on expected shell output for 'wow' being input\n\
    // at the command line. Currently it supports output from bash, zsh, fish, cmd and powershell.\n\t\
    function detectWowCommand(data) {\n\
        const patterns = [\n\t\t\
            'wow: command not found',\n\t\t\
            'command not found: wow',\n\t\t\
            'Unknown command \\'wow\\'',\n\t\t\
            '\\'wow\\' is not recognized*',\n\t\t\
            '\\'wow\\'은\\\\(는\\\\) 내부 또는 외부 명령.*',\n\t\t\
            'Command \\'wow\\' not found, did you mean:'\n\t\
        ];\n\t\
        return new RegExp('(' + patterns.join(')|(') + ')').test(data)\n\
    }\n\n\
    
    // Our extension's custom ui state reducer. Here we can listen for our 'WOW_MODE_TOGGLE' action\n\
    // and modify the state accordingly.\n\
    exports.reduceUI = (state, action) => {\n\t\
        switch (action.type) {\n\t\t\
            case 'WOW_MODE_TOGGLE':\n\t\t\t\
                // Toggle wow mode!\n\t\t\t\
                return state.set('wowMode', !state.wowMode);\n\t\
        }\n\t\
        return state;\n\
    };\n\n\
    
    // Our extension's state property mapper. Here we can pass the ui's `wowMode` state\n\
    // into the terminal component's properties.\n\
    exports.mapTermsState = (state, map) => {\n\t\
        return Object.assign(map, {\n\t\t\
            wowMode: state.ui.wowMode\n\t\
        });\n\
    };\n\n\
    // We'll need to handle reflecting the `wowMode` property down through possible nested\n\
    // parent/children terminal hierarchies.\n\
    const passProps = (uid, parentProps, props) => {\n\t\
        return Object.assign(props, {\n\t\t\
            wowMode: parentProps.wowMode\n\t\
        });\n\
    }\n\n\
   
    exports.getTermGroupProps = passProps;\n\
    exports.getTermProps = passProps;\n\n\
    
    // The `decorateTerm` hook allows our extension to return a higher order react component.\n\
    // It supplies us with:\n\
    // - Term: The terminal component.\n\
    // - React: The enture React namespace.\n\
    // - notify: Helper function for displaying notifications in the operating system.\n\
    //\n\
    // The portions of this code dealing with the particle simulation are heavily based on:\n\
    // - https://atom.io/packages/power-mode\n\
    // - https://github.com/itszero/rage-power/blob/master/index.jsx\n\
    exports.decorateTerm = (Term, { React, notify }) => {\n\t\
        // Define and return our higher order component.\n\t\
        return class extends React.Component {\n\t\t\
            constructor(props, context) {\n\t\t\t\
                super(props, context);\n\t\t\t\
                // Since we'll be passing these functions around, we need to bind this\n\t\t\t\
                // to each.\n\t\t\t\
                this._drawFrame = this._drawFrame.bind(this);\n\t\t\t\
                this._resizeCanvas = this._resizeCanvas.bind(this);\n\t\t\t\
                this._onDecorated = this._onDecorated.bind(this);\n\t\t\t\
                this._onCursorMove = this._onCursorMove.bind(this);\n\t\t\t\
                this._shake = throttle(this._shake.bind(this), 100, { trailing: false });\n\t\t\t\
                this._spawnParticles = throttle(this._spawnParticles.bind(this), 25, { trailing: false });\n\t\t\t\
                // Initial particle state\n\t\t\t\
                this._particles = [];\n\t\t\t\
                // We'll set these up when the terminal is available in `_onDecorated`\n\t\t\t\
                this._div = null;\n\t\t\t\
                this._canvas = null;\n\t\t\
            }\n\n\t\t\
            _onDecorated(term) {\n\t\t\t\
                if (this.props.onDecorated) this.props.onDecorated(term);\n\t\t\t\
                this._div = term ? term.termRef : null;\n\t\t\t\
                this._initCanvas();\n\t\t\
            }\n\n\t\t\
            // Set up our canvas element we'll use to do particle effects on.\n\t\t\
            _initCanvas() {\n\t\t\t\
                this._canvas = document.createElement('canvas');\n\t\t\t\
                this._canvas.style.position = 'absolute';\n\t\t\t\
                this._canvas.style.top = '0';\n\t\t\t\
                this._canvas.style.pointerEvents = 'none';\n\t\t\t\
                this._canvasContext = this._canvas.getContext('2d');\n\t\t\t\
                this._canvas.width = window.innerWidth;\n\t\t\t\
                this._canvas.height = window.innerHeight;\n\t\t\t\
                document.body.appendChild(this._canvas);\n\t\t\t\
                window.requestAnimationFrame(this._drawFrame);\n\t\t\t\
                window.addEventListener('resize', this._resizeCanvas);\n\t\t\
            }\n\n\t\t\
            _resizeCanvas() {\n\t\t\t\
                this._canvas.width = window.innerWidth;\n\t\t\t\
                this._canvas.height = window.innerHeight;\n\t\t\
            }\n\n\t\t\
            // Draw the next frame in the particle simulation.\n\t\t\
            _drawFrame() {\n\t\t\t\
                this._particles.length && this._canvasContext.clearRect(0, 0, this._canvas.width, this._canvas.height);\n\t\t\t\
                this._particles.forEach((particle) => {\n\t\t\t\t\
                    particle.velocity.y += PARTICLE_GRAVITY;\n\t\t\t\t\
                    particle.x += particle.velocity.x;\n\t\t\t\t\
                    particle.y += particle.velocity.y;\n\t\t\t\t\
                    particle.alpha *= PARTICLE_ALPHA_FADEOUT;\n\t\t\t\t\
                    if (particle.alpha > PARTICLE_ALPHA_MIN_THRESHOLD) {\n\t\t\t\t\t\
                        this._canvasContext.fillRect(Math.round(particle.x - 1), Math.round(particle.y - 1), 3, 3);\n\t\t\t\t\t\
                        this._canvasContext.fillStyle = `rgba(${particle.color.join(',')}, ${particle.alpha})`;\n\t\t\t\t\t\
                        this._canvasContext.fillRect(Math.round(particle.x - 1), Math.round(particle.y - 1), 3, 3);\n\t\t\t\t\
                    }\n\
                });\n\t\t\t\
                this._particles = this._particles\n\t\t\t\t\
                    .slice(Math.max(this._particles.length - MAX_PARTICLES, 0))\n\t\t\t\t\
                    .filter((particle) => particle.alpha > PARTICLE_ALPHA_MIN_THRESHOLD);\n\t\t\t\
                if (this._particles.length > 0 || this.props.needsRedraw) {\n\t\t\t\t\
                    window.requestAnimationFrame(this._drawFrame);\n\t\t\t\
                }\n\t\t\t\
                this.props.needsRedraw = this._particles.length === 0;\n\t\t\
            }\n\n\t\t\
            // Pushes `PARTICLE_NUM_RANGE` new particles into the simulation.\n\t\t\
            _spawnParticles(x, y) {\n\t\t\t\
                // const { colors } = this.props;\n\t\t\t\
                const length = this._particles.length;\n\t\t\t\
                const colors = this.props.wowMode ?\n\t\t\t\t\
                    values(this.props.colors).map(toHex) :\n\t\t\t\t\
                    [toHex(this.props.cursorColor)];\n\t\t\t\
                const numParticles = PARTICLE_NUM_RANGE();\n\t\t\t\
                for (let i = 0; i < numParticles; i++) {\n\t\t\t\t\
                    const colorCode = colors[i % colors.length];\n\t\t\t\t\
                    const r = parseInt(colorCode.slice(1, 3), 16);\n\t\t\t\t\
                    const g = parseInt(colorCode.slice(3, 5), 16);\n\t\t\t\t\
                    const b = parseInt(colorCode.slice(5, 7), 16);\n\t\t\t\t\
                    const color = [r, g, b];\n\t\t\t\t\
                    this._particles.push(this._createParticle(x, y, color));\n\t\t\t\
                }\n\t\t\t\
                if (length === 0) {\n\t\t\t\t\
                    window.requestAnimationFrame(this._drawFrame);\n\t\t\t\
                }\n\t\t\
            }\n\n\t\t\
            // Returns a particle of a specified color\n\t\t\
            // at some random offset from the input coordinates.\n\t\t\
            _createParticle(x, y, color) {\n\t\t\t\
                return {\n\t\t\t\
                    x,\n\t\t\t\t\
                    y: y,\n\t\t\t\t\
                    alpha: 1,\n\t\t\t\t\
                    color,\n\t\t\t\t\
                    velocity: {\n\t\t\t\t\t\
                        x: PARTICLE_VELOCITY_RANGE.x[0] + Math.random() *\n\t\t\t\t\t\t\
                            (PARTICLE_VELOCITY_RANGE.x[1] - PARTICLE_VELOCITY_RANGE.x[0]),\n\t\t\t\t\t\t\t\
                        y: PARTICLE_VELOCITY_RANGE.y[0] + Math.random() *\n\t\t\t\t\t\t\
                            (PARTICLE_VELOCITY_RANGE.y[1] - PARTICLE_VELOCITY_RANGE.y[0])\n\t\t\t\t\
                    }\n\t\t\t\
                };\n\t\t\
            }\n\n\t\t\
            // 'Shakes' the screen by applying a temporary translation\n\t\t\
            // to the terminal container.\n\t\t\
            _shake() {\n\t\t\t\
                // TODO: Maybe we should do this check in `_onCursorMove`?\n\t\t\t\
                if (!this.props.wowMode) return;\n\n\t\t\t\
                const intensity = 1 + 2 * Math.random();\n\t\
                const x = intensity * (Math.random() > 0.5 ? -1 : 1);\n\t\t\t\
                const y = intensity * (Math.random() > 0.5 ? -1 : 1);\n\t\t\t\
                this._div.style.transform = `translate3d(${x}px, ${y}px, 0)`;\n\t\t\t\
                setTimeout(() => {\n\t\t\t\t\
                    if (this._div) this._div.style.transform = '';\n\t\t\t\
                }, 75);\n\t\t\
            }\n\n\t\t\
            _onCursorMove(cursorFrame) {\n\t\t\t\
                if (this.props.onCursorMove) this.props.onCursorMove(cursorFrame);\n\t\t\t\
                this._shake();\n\t\t\t\
                const { x, y } = cursorFrame;\n\t\t\t\
                const origin = this._div.getBoundingClientRect();\n\t\t\t\
                requestAnimationFrame(() => {\n\t\t\t\t\
                    this._spawnParticles(x + origin.left, y + origin.top);\n\t\t\t\
                });\n\t\t\
            }\n\n\t\t\
            // Called when the props change, here we'll check if wow mode has gone\n\t\t\
            // on -> off or off -> on and notify the user accordingly.\n\t\t\
            componentWillReceiveProps(next) {\n\t\t\t\
                if (next.wowMode && !this.props.wowMode) {\n\t\t\t\t\
                    notify('WOW such on');\n\t\t\t\
                } else if (!next.wowMode && this.props.wowMode) {\n\t\t\t\t\
                    notify('WOW such off');\n\t\t\t\
                }\n\t\t\
            }\n\n\t\t\
            render() {\n\t\t\t\
                // Return the default Term component with our custom onTerminal closure\n\t\t\t\
                // setting up and managing the particle effects.\n\t\t\t\
                return React.createElement(Term, Object.assign({}, this.props, {\n\t\t\t\t\
                    onDecorated: this._onDecorated,\n\t\t\t\t\
                    onCursorMove: this._onCursorMove\n\t\t\t\
                }));\n\t\t\
            }\n\n\t\t\
            componentWillUnmount() {\n\t\t\t\
                document.body.removeChild(this._canvas);\n\t\t\
            }\n\t\
        }\n\
    };\n";
    // break_point(0,type_of(path));
    let file = File::create(&path.as_ref() as &str);
    if file.is_err() {
        let string: &str = "could not be opened.";
        println!(" {} {}",
            path .blue().bold(),
            string.red().bold()
            );
        exit(2);
    };
    let mut file = file.unwrap();
    match file.write_all(&FILE_TEXT.as_bytes()) {
        Err(_) => {
            let string: &str = "could not be patched.";
            println!(" {} {}",
                path.blue().bold(),
                string.red().bold()
                );
            exit(1);
        },
        _ => {
            let string: &str = "was successfully patched.";
            println!(" {} {}",
                path.blue().bold(),
                string.green().bold()
                );
        }
    }
}
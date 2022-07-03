var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

function _possibleConstructorReturn(self, call) { if (!self) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return call && (typeof call === "object" || typeof call === "function") ? call : self; }

function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function, not " + typeof superClass); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, enumerable: false, writable: true, configurable: true } }); if (superClass) Object.setPrototypeOf ? Object.setPrototypeOf(subClass, superClass) : subClass.__proto__ = superClass; }

//FIXME: SE ROMPIO QUERIENDO HACER EL STOPWATCH

var Clock = function (_React$Component) {
    _inherits(Clock, _React$Component);

    function Clock(props) {
        _classCallCheck(this, Clock);

        var _this = _possibleConstructorReturn(this, (Clock.__proto__ || Object.getPrototypeOf(Clock)).call(this, props));

        _this.state = { date: new Date(Date.now()).toTimeString().split(' ')[0],
            last_pressed: false,
            current: false
        };
        _this.last_pressed = false;
        _this.current = false;
        //binds
        _this.handleTick = _this.handleTick.bind(_this);
        return _this;
    }

    _createClass(Clock, [{
        key: 'componentDidMount',
        value: function componentDidMount() {
            this.start();
        }
    }, {
        key: 'handleTick',
        value: function handleTick() {
            console.log('tick');
            this.setState({
                date: new Date(Date.now()).toTimeString().split(' ')[0]
            });
        }
    }, {
        key: 'stop',
        value: function stop() {
            console.log('clock stopped...');
            clearInterval(this.state.ticker);
            /*this.setState({
                last_pressed: this.props.pressed,
                current: false
            })*/
            this.last_pressed = this.props.pressed;
            this.current = false;
        }
    }, {
        key: 'start',
        value: function start() {
            console.log('clock started...');
            this.setState({
                ticker: setInterval(this.handleTick, 1000),
                last_pressed: this.props.pressed,
                current: true
            });
            this.last_pressed = this.props.pressed;
            this.current = true;
        }
    }, {
        key: 'render',
        value: function render() {
            if (!this.state.last_pressed && this.props.pressed) {
                this.stop();
            }
            if (this.current == false) {
                this.start();
            }
            return React.createElement(
                'p',
                null,
                this.state.date
            );
        }
    }]);

    return Clock;
}(React.Component);

export default Clock;
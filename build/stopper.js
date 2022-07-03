var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

function _possibleConstructorReturn(self, call) { if (!self) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return call && (typeof call === "object" || typeof call === "function") ? call : self; }

function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function, not " + typeof superClass); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, enumerable: false, writable: true, configurable: true } }); if (superClass) Object.setPrototypeOf ? Object.setPrototypeOf(subClass, superClass) : subClass.__proto__ = superClass; }

import Clock from './clock.js';
import ButtonToggler from './btn_toggler.js';
//TODO: INVESTIGAR COMO HACER UN STOPWATCH EN REACT

var Stopper = function (_React$Component) {
    _inherits(Stopper, _React$Component);

    function Stopper(props) {
        _classCallCheck(this, Stopper);

        var _this = _possibleConstructorReturn(this, (Stopper.__proto__ || Object.getPrototypeOf(Stopper)).call(this, props));

        _this.state = {
            pressed: false
        };

        _this.handleClick = _this.handleClick.bind(_this);
        return _this;
    }

    _createClass(Stopper, [{
        key: 'handleClick',
        value: function handleClick() {
            this.setState(function (prev, props) {
                return { pressed: !prev.pressed };
            });
            console.log('pressed: ' + this.state.pressed);
        }
    }, {
        key: 'render',
        value: function render() {
            console.log('estado pressed: ' + this.state.pressed);
            return React.createElement(
                'div',
                null,
                React.createElement(Clock, { pressed: this.state.pressed }),
                React.createElement(ButtonToggler, { onClick: this.handleClick, messageOn: 'On', messageOff: 'Off' })
            );
        }
    }]);

    return Stopper;
}(React.Component);

export default Stopper;
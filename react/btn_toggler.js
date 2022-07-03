
class ButtonToggler extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            on: false
        }

        this.handleClick = this.handleClick.bind(this)
    }

    handleClick() {
        this.setState((prev, props) => ({
            on: !prev.on
        }));
        return this.props.onClick()
    }

    render() {
        if(this.state.on) {
            return (<button onClick={this.handleClick}>{this.props.messageOn}</button>)
        } else {
            return (<button onClick={this.handleClick}>{this.props.messageOff}</button>)
        }
    }
}

export default ButtonToggler
import Clock from './clock.js'
import ButtonToggler from './btn_toggler.js'
//TODO: INVESTIGAR COMO HACER UN STOPWATCH EN REACT
class Stopper extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            pressed: false
        }

        this.handleClick = this.handleClick.bind(this)
    }

    handleClick() {
        this.setState(
            (prev, props) => (
                {pressed: !prev.pressed}
            )
        )
        console.log(`pressed: ${this.state.pressed}`)
    }

    render() {
        console.log(`estado pressed: ${this.state.pressed}`)
        return (
            <div>
                <Clock pressed={this.state.pressed}/>
                <ButtonToggler onClick={this.handleClick} messageOn="On" messageOff="Off"/>
            </div>
            
        )
    }
}

export default Stopper
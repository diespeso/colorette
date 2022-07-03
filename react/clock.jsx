//FIXME: SE ROMPIO QUERIENDO HACER EL STOPWATCH

class Clock extends React.Component {
    constructor(props) {
        super(props)
        this.state = {date: new Date(Date.now()).toTimeString().split(' ')[0],
            last_pressed: false,
            current: false,
        }
        this.last_pressed = false;
        this.current = false;
        //binds
        this.handleTick = this.handleTick.bind(this)
    }

    componentDidMount() {
        this.start()
    }

    handleTick() {
        console.log('tick')
        this.setState({
            date: new Date(Date.now()).toTimeString().split(' ')[0]
        })
    }

    stop() {
        console.log('clock stopped...')
        clearInterval(this.state.ticker)
        /*this.setState({
            last_pressed: this.props.pressed,
            current: false
        })*/
        this.last_pressed = this.props.pressed
        this.current = false
    }

    start() {
        console.log('clock started...')
        this.setState({
            ticker: setInterval(this.handleTick, 1000),
            last_pressed: this.props.pressed,
            current: true
        })
        this.last_pressed = this.props.pressed
        this.current = true
    }

    render() {
        if(!this.state.last_pressed && this.props.pressed) {
            this.stop()
        }
        if(this.current == false) {
            this.start()
        }
        return (
            <p>{this.state.date}</p>
        )
    }
}

export default Clock
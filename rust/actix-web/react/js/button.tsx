type ButtonProps = {
    onClick: () => void,
    text: string,
};

class Button extends React.Component<ButtonProps> {
    constructor(props: ButtonProps) {
        super(props);
        this.handleClick = this.handleClick.bind(this);
    }

    handleClick() {
        this.props.onClick();
    }

    render() {
        console.log("Render Button");
        return (<button onClick={this.handleClick}>{this.props.text}</button>);
    }
}

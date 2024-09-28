class Root extends React.Component {
    render() {
        console.log("Render Root");
        return (<div>
            <h1>React Example</h1>
            <Form name="Alice" age="20"/>
            <Table/>
        </div>);
    }
}

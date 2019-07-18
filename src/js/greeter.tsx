import * as React from "react";
import ApolloClient from "apollo-boost";
import { ApolloProvider } from "react-apollo";
import { Pane, Heading } from "evergreen-ui";

const client = new ApolloClient();

function Hello({ name }: { name: string }) {
  return <Heading>Hello {name}!!</Heading>;
}

export default function App() {
  return (
    <ApolloProvider client={client}>
      <Pane width={1024} margin="auto">
        <Hello name="Apollo" />
      </Pane>
    </ApolloProvider>
  );
}

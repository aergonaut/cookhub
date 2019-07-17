import * as React from "react";
import ApolloClient from "apollo-boost";
import { ApolloProvider } from "react-apollo";

const client = new ApolloClient();

function Hello({ name }: { name: string }) {
  return <h1>Hello {name}!!</h1>;
}

export default function App() {
  return (
    <ApolloProvider client={client}>
      <Hello name="Apollo" />
    </ApolloProvider>
  );
}

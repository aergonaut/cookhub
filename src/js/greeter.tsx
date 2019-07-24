import * as React from "react";
import ApolloClient, { gql } from "apollo-boost";
import { ApolloProvider, Query } from "react-apollo";
import { Pane, Heading, Text, Link, majorScale } from "evergreen-ui";
import { RecipesQuery } from "./__generated__/RecipesQuery";

const client = new ApolloClient();

const RECIPES_QUERY = gql`
  query RecipesQuery {
    recipes {
      id
      title
      sourceUrl
      opengraph {
        siteName
        imageUrl
      }
    }
  }
`;

export default function App() {
  return (
    <ApolloProvider client={client}>
      <Pane
        display="flex"
        position="sticky"
        top={0}
        elevation={1}
        padding={majorScale(2)}
        width="100%"
        marginBottom={majorScale(2)}
      >
        <Heading is="h1">Cookhub</Heading>
      </Pane>
      <Pane width={1024} margin="auto" padding={majorScale(2)}>
        <Query<RecipesQuery> query={RECIPES_QUERY}>
          {({ loading, error, data }) => {
            if (loading) return <Text>Loading...</Text>;
            if (error) return <Text>Error...</Text>;

            return data.recipes.map(recipe => (
              <Pane key={recipe.id} border width="33%">
                <Pane
                  height={majorScale(24)}
                  backgroundImage={`url("${recipe.opengraph.imageUrl}")`}
                  backgroundSize="cover"
                  backgroundPosition="center"
                />
                <Pane padding={majorScale(2)}>
                  <Heading size={600}>{recipe.title}</Heading>
                </Pane>
                <Pane padding={majorScale(2)}>
                  <Link href={recipe.sourceUrl} color="muted">
                    {recipe.opengraph.siteName}
                  </Link>
                </Pane>
              </Pane>
            ));
          }}
        </Query>
      </Pane>
    </ApolloProvider>
  );
}

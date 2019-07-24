/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: RecipesQuery
// ====================================================

export interface RecipesQuery_recipes_opengraph {
  __typename: "OpenGraphInfo";
  /**
   * The website's site name
   */
  siteName: string | null;
  /**
   * The website's image
   */
  imageUrl: string | null;
}

export interface RecipesQuery_recipes {
  __typename: "Recipe";
  id: any;
  title: string;
  sourceUrl: string | null;
  opengraph: RecipesQuery_recipes_opengraph | null;
}

export interface RecipesQuery {
  recipes: RecipesQuery_recipes[];
}

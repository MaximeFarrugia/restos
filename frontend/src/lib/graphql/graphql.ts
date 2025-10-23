/* eslint-disable */
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = T | null | undefined;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type Address = {
  __typename?: 'Address';
  address: Scalars['String']['output'];
  city: Scalars['String']['output'];
  complement?: Maybe<Scalars['String']['output']>;
  country: Scalars['String']['output'];
  zipCode: Scalars['String']['output'];
};

export type AddressInput = {
  address: Scalars['String']['input'];
  city: Scalars['String']['input'];
  complement?: InputMaybe<Scalars['String']['input']>;
  country: Scalars['String']['input'];
  zipCode: Scalars['String']['input'];
};

export type Contact = {
  __typename?: 'Contact';
  email?: Maybe<Scalars['String']['output']>;
  phoneNumber?: Maybe<Scalars['String']['output']>;
};

export type Item = {
  __typename?: 'Item';
  allergens: Array<Scalars['String']['output']>;
  brand?: Maybe<Scalars['String']['output']>;
  category?: Maybe<Scalars['String']['output']>;
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  quantityUnit: Scalars['String']['output'];
  type: Scalars['String']['output'];
};

export type LoginInput = {
  email: Scalars['String']['input'];
  password: Scalars['String']['input'];
};

export type LoginPayload = {
  __typename?: 'LoginPayload';
  success: Scalars['Boolean']['output'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  login: LoginPayload;
  register: User;
};


export type MutationRootLoginArgs = {
  input: LoginInput;
};


export type MutationRootRegisterArgs = {
  input: RegisterInput;
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  items: Array<Item>;
};

export type RegisterInput = {
  email: Scalars['String']['input'];
  firstName: Scalars['String']['input'];
  lastName: Scalars['String']['input'];
  password: Scalars['String']['input'];
  phoneNumber?: InputMaybe<Scalars['String']['input']>;
  restaurantAddress: AddressInput;
  restaurantContactEmail?: InputMaybe<Scalars['String']['input']>;
  restaurantContactPhoneNumber?: InputMaybe<Scalars['String']['input']>;
  restaurantName: Scalars['String']['input'];
};

export type Restaurant = {
  __typename?: 'Restaurant';
  address: Address;
  contact: Contact;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  parent?: Maybe<Restaurant>;
};

export type User = {
  __typename?: 'User';
  email: Scalars['String']['output'];
  firstName: Scalars['String']['output'];
  id: Scalars['ID']['output'];
  lastName: Scalars['String']['output'];
  phoneNumber?: Maybe<Scalars['String']['output']>;
  restaurant?: Maybe<Restaurant>;
  role: UserRole;
};

export enum UserRole {
  Owner = 'OWNER'
}

export type LoginMutationMutationVariables = Exact<{
  input: LoginInput;
}>;


export type LoginMutationMutation = { __typename?: 'MutationRoot', login: { __typename?: 'LoginPayload', success: boolean } };


export const LoginMutationDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"LoginMutation"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"LoginInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"login"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"success"}}]}}]}}]} as unknown as DocumentNode<LoginMutationMutation, LoginMutationMutationVariables>;
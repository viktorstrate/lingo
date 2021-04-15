module App exposing (..)

import Api.Query as Query exposing (apiVersion)
import Browser
import Graphql.Http
import Graphql.Operation exposing (RootQuery)
import Graphql.SelectionSet exposing (SelectionSet, with)
import Html exposing (..)


type alias Response =
    String


type Msg
    = GotResponse (Result (Graphql.Http.Error Response) Response)


type alias Model =
    { version : Maybe String
    , error : Maybe String
    }


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


init : () -> ( Model, Cmd Msg )
init =
    \_ -> ( Model Nothing Nothing, makeRequest )


view : Model -> Html Msg
view model =
    case model.error of
        Just err ->
            text ("got error: " ++ err)

        Nothing ->
            case model.version of
                Just v ->
                    text v

                Nothing ->
                    text "No version yet..."


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotResponse response ->
            case response of
                Ok val ->
                    ( { model | version = Just val }, Cmd.none )

                Err err ->
                    ( { model | error = Just "error" }, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



--- Graphql
-- query : SelectionSet Response RootQuery
-- query =
--     Query.apiVersion Response
--         |> with (Query { login = "octocat" } user)


query : SelectionSet String RootQuery
query =
    Query.apiVersion


makeRequest : Cmd Msg
makeRequest =
    query |> Graphql.Http.queryRequest "http://localhost:8080/graphql" |> Graphql.Http.send GotResponse

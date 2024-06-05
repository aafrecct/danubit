module Pages.Home_ exposing (Model, Msg, page)

import Html
import Http
import Common.CommonUI exposing (siteHeader)
import View exposing (View)
import Page exposing (Page)
import Request exposing (Request)
import Shared

page : Shared.Model -> Request -> Page.With Model Msg
page shared request =
  Page.element 
  { init = init shared
  , update = update
  , view = view
  , subscriptions = subscriptions
  }
  
type Msg 
  = GotAsociations ( Result Http.Error String)

type Model
  = Failure
  | Loading
  | Success


init : Shared.Model -> ( Model, Cmd Msg )
init _ = 
  ( Loading
  , Http.get
    { url = "https://localhost:2345/api/asociations"
    , expect = Http.expectString GotAsociations
    }
  )

update : Msg -> Model -> ( Model, Cmd Msg )
update _ _ = 
  ( Success 
  , Cmd.none
  )

subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none

view : Model -> View Msg
view _ =
  { title = "Danubit"
  , body = [ siteHeader "Asociaciones" ]
  }


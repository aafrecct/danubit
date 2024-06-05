module Common.CommonUI exposing ( siteHeader )

import Html exposing ( div, text, a, header, nav, img)
import Html.Attributes exposing ( id, class, src, href )

siteNavItem activeItemName item =
  let 
    (itemName, itemHref) = item
    upperItemName = String.toUpper itemName
    active = activeItemName == itemName
  in 
    a 
    [ id ( "site-nav-" ++ itemHref )
    , class ( "site-nav-item" ++ if active then " active" else "" )
    , href ( if active then "" else ( "/" ++ itemHref ) ) 
    ]
    [ text upperItemName ]

siteHeader activePage = 
  header [ id "site-header", class "header" ] 
  [ div  [ id "site-header-right", class "header-block"]
    [ a [ href "/" ] 
      [ img [ id "site-logo", class "logo", src "static/img/logo.svg" ] []]
    , nav [ id "site-navigation" ] ( List.map (siteNavItem activePage)
      [ ( "Asociaciones", "asociaciones" )
      , ( "Actividades",  "actividades" )
      , ( "Convivencia",  "convivencia" )
      , ( "Información",  "informacion" ) 
      ])
    ]
  , div  [ id "site-header-left", class "header-block"] []
  ]


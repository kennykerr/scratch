pub mod windows
{
    pub mod ui
    {
        # [repr (C)] # [derive (Default, Debug, PartialEq)] pub struct Color
        { pub r#a : u8, pub r#r : u8, pub r#g : u8, pub r#b : u8, } pub struct
        ColorHelper { ptr : * mut std :: ffi :: c_void } impl ColorHelper
        {
            pub fn r#to_display_name (color : & Color,) -> winrt :: Result <
            winrt :: String >
            {
                winrt :: factory :: < ColorHelper, IColorHelperStatics2 > () ?
                . r#to_display_name (color)
            } pub fn r#from_argb (a : u8, r : u8, g : u8, b : u8,) -> winrt ::
            Result < Color >
            {
                winrt :: factory :: < ColorHelper, IColorHelperStatics > () ?
                . r#from_argb (a, r, g, b)
            }
        } impl winrt :: TypeName for ColorHelper
        { fn type_name () -> & 'static str { "Windows.UI.ColorHelper" } } impl
        winrt :: AsAbi for ColorHelper
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for ColorHelper
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } pub struct Colors { ptr : * mut std :: ffi :: c_void } impl Colors
        {
            pub fn r#alice_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#alice_blue ()
            } pub fn r#antique_white () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#antique_white ()
            } pub fn r#aqua () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#aqua
                ()
            } pub fn r#aquamarine () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#aquamarine ()
            } pub fn r#azure () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#azure
                ()
            } pub fn r#beige () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#beige
                ()
            } pub fn r#bisque () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#bisque
                ()
            } pub fn r#black () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#black
                ()
            } pub fn r#blanched_almond () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#blanched_almond ()
            } pub fn r#blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#blue
                ()
            } pub fn r#blue_violet () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#blue_violet ()
            } pub fn r#brown () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#brown
                ()
            } pub fn r#burly_wood () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#burly_wood ()
            } pub fn r#cadet_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#cadet_blue ()
            } pub fn r#chartreuse () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#chartreuse ()
            } pub fn r#chocolate () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#chocolate ()
            } pub fn r#coral () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#coral
                ()
            } pub fn r#cornflower_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#cornflower_blue ()
            } pub fn r#cornsilk () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#cornsilk ()
            } pub fn r#crimson () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#crimson ()
            } pub fn r#cyan () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#cyan
                ()
            } pub fn r#dark_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_blue ()
            } pub fn r#dark_cyan () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_cyan ()
            } pub fn r#dark_goldenrod () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_goldenrod ()
            } pub fn r#dark_gray () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_gray ()
            } pub fn r#dark_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_green ()
            } pub fn r#dark_khaki () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_khaki ()
            } pub fn r#dark_magenta () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_magenta ()
            } pub fn r#dark_olive_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_olive_green ()
            } pub fn r#dark_orange () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_orange ()
            } pub fn r#dark_orchid () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_orchid ()
            } pub fn r#dark_red () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_red ()
            } pub fn r#dark_salmon () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_salmon ()
            } pub fn r#dark_sea_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_sea_green ()
            } pub fn r#dark_slate_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_slate_blue ()
            } pub fn r#dark_slate_gray () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_slate_gray ()
            } pub fn r#dark_turquoise () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_turquoise ()
            } pub fn r#dark_violet () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dark_violet ()
            } pub fn r#deep_pink () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#deep_pink ()
            } pub fn r#deep_sky_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#deep_sky_blue ()
            } pub fn r#dim_gray () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dim_gray ()
            } pub fn r#dodger_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#dodger_blue ()
            } pub fn r#firebrick () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#firebrick ()
            } pub fn r#floral_white () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#floral_white ()
            } pub fn r#forest_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#forest_green ()
            } pub fn r#fuchsia () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#fuchsia ()
            } pub fn r#gainsboro () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#gainsboro ()
            } pub fn r#ghost_white () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#ghost_white ()
            } pub fn r#gold () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#gold
                ()
            } pub fn r#goldenrod () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#goldenrod ()
            } pub fn r#gray () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#gray
                ()
            } pub fn r#green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#green
                ()
            } pub fn r#green_yellow () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#green_yellow ()
            } pub fn r#honeydew () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#honeydew ()
            } pub fn r#hot_pink () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#hot_pink ()
            } pub fn r#indian_red () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#indian_red ()
            } pub fn r#indigo () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#indigo
                ()
            } pub fn r#ivory () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#ivory
                ()
            } pub fn r#khaki () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#khaki
                ()
            } pub fn r#lavender () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#lavender ()
            } pub fn r#lavender_blush () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#lavender_blush ()
            } pub fn r#lawn_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#lawn_green ()
            } pub fn r#lemon_chiffon () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#lemon_chiffon ()
            } pub fn r#light_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_blue ()
            } pub fn r#light_coral () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_coral ()
            } pub fn r#light_cyan () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_cyan ()
            } pub fn r#light_goldenrod_yellow () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_goldenrod_yellow ()
            } pub fn r#light_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_green ()
            } pub fn r#light_gray () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_gray ()
            } pub fn r#light_pink () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_pink ()
            } pub fn r#light_salmon () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_salmon ()
            } pub fn r#light_sea_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_sea_green ()
            } pub fn r#light_sky_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_sky_blue ()
            } pub fn r#light_slate_gray () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_slate_gray ()
            } pub fn r#light_steel_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_steel_blue ()
            } pub fn r#light_yellow () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#light_yellow ()
            } pub fn r#lime () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#lime
                ()
            } pub fn r#lime_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#lime_green ()
            } pub fn r#linen () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#linen
                ()
            } pub fn r#magenta () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#magenta ()
            } pub fn r#maroon () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#maroon
                ()
            } pub fn r#medium_aquamarine () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_aquamarine ()
            } pub fn r#medium_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_blue ()
            } pub fn r#medium_orchid () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_orchid ()
            } pub fn r#medium_purple () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_purple ()
            } pub fn r#medium_sea_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_sea_green ()
            } pub fn r#medium_slate_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_slate_blue ()
            } pub fn r#medium_spring_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_spring_green ()
            } pub fn r#medium_turquoise () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_turquoise ()
            } pub fn r#medium_violet_red () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#medium_violet_red ()
            } pub fn r#midnight_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#midnight_blue ()
            } pub fn r#mint_cream () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#mint_cream ()
            } pub fn r#misty_rose () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#misty_rose ()
            } pub fn r#moccasin () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#moccasin ()
            } pub fn r#navajo_white () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#navajo_white ()
            } pub fn r#navy () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#navy
                ()
            } pub fn r#old_lace () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#old_lace ()
            } pub fn r#olive () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#olive
                ()
            } pub fn r#olive_drab () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#olive_drab ()
            } pub fn r#orange () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#orange
                ()
            } pub fn r#orange_red () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#orange_red ()
            } pub fn r#orchid () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#orchid
                ()
            } pub fn r#pale_goldenrod () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#pale_goldenrod ()
            } pub fn r#pale_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#pale_green ()
            } pub fn r#pale_turquoise () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#pale_turquoise ()
            } pub fn r#pale_violet_red () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#pale_violet_red ()
            } pub fn r#papaya_whip () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#papaya_whip ()
            } pub fn r#peach_puff () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#peach_puff ()
            } pub fn r#peru () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#peru
                ()
            } pub fn r#pink () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#pink
                ()
            } pub fn r#plum () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#plum
                ()
            } pub fn r#powder_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#powder_blue ()
            } pub fn r#purple () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#purple
                ()
            } pub fn r#red () -> winrt :: Result < Color >
            { winrt :: factory :: < Colors, IColorsStatics > () ? . r#red () }
            pub fn r#rosy_brown () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#rosy_brown ()
            } pub fn r#royal_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#royal_blue ()
            } pub fn r#saddle_brown () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#saddle_brown ()
            } pub fn r#salmon () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#salmon
                ()
            } pub fn r#sandy_brown () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#sandy_brown ()
            } pub fn r#sea_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#sea_green ()
            } pub fn r#sea_shell () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#sea_shell ()
            } pub fn r#sienna () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#sienna
                ()
            } pub fn r#silver () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#silver
                ()
            } pub fn r#sky_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#sky_blue ()
            } pub fn r#slate_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#slate_blue ()
            } pub fn r#slate_gray () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#slate_gray ()
            } pub fn r#snow () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#snow
                ()
            } pub fn r#spring_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#spring_green ()
            } pub fn r#steel_blue () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#steel_blue ()
            } pub fn r#tan () -> winrt :: Result < Color >
            { winrt :: factory :: < Colors, IColorsStatics > () ? . r#tan () }
            pub fn r#teal () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#teal
                ()
            } pub fn r#thistle () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#thistle ()
            } pub fn r#tomato () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#tomato
                ()
            } pub fn r#transparent () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#transparent ()
            } pub fn r#turquoise () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#turquoise ()
            } pub fn r#violet () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#violet
                ()
            } pub fn r#wheat () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#wheat
                ()
            } pub fn r#white () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#white
                ()
            } pub fn r#white_smoke () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#white_smoke ()
            } pub fn r#yellow () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? . r#yellow
                ()
            } pub fn r#yellow_green () -> winrt :: Result < Color >
            {
                winrt :: factory :: < Colors, IColorsStatics > () ? .
                r#yellow_green ()
            }
        } impl winrt :: TypeName for Colors
        { fn type_name () -> & 'static str { "Windows.UI.Colors" } } impl
        winrt :: AsAbi for Colors
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for Colors
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } # [repr (C)] pub struct IColorHelper
        { ptr : * mut std :: ffi :: c_void } # [repr (C)] struct
        abi_IColorHelper
        {
            __0 : usize, __1 : usize, __2 : usize, __3 : usize, __4 : usize,
            __5 : usize,
        } impl IColorHelper { } impl winrt :: TypeGuid for IColorHelper
        {
            fn type_guid () -> & 'static winrt :: Guid
            {
                static GUID : winrt :: Guid = winrt :: Guid :: from_values
                (423427047, 26055, 17728, &
                 [173, 8, 98, 131, 186, 118, 135, 154],) ; & GUID
            }
        } impl winrt :: AsAbi for IColorHelper
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for IColorHelper
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } # [repr (C)] pub struct IColorHelperStatics
        { ptr : * mut std :: ffi :: c_void } # [repr (C)] struct
        abi_IColorHelperStatics
        {
            __0 : usize, __1 : usize, __2 : usize, __3 : usize, __4 : usize,
            __5 : usize, r#from_argb : extern "system" fn
            (* const std :: ffi :: c_void, u8, u8, u8, u8, & mut Color,) ->
            winrt :: ErrorCode,
        } impl IColorHelperStatics
        {
            pub fn r#from_argb (& self, a : u8, r : u8, g : u8, b : u8,) ->
            winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (*
                       (self . ptr as * const * const
                        abi_IColorHelperStatics))) . r#from_argb)
                    (self . ptr, a, r, g, b, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            }
        } impl winrt :: TypeGuid for IColorHelperStatics
        {
            fn type_guid () -> & 'static winrt :: Guid
            {
                static GUID : winrt :: Guid = winrt :: Guid :: from_values
                (2231688170, 64362, 16708, &
                 [166, 194, 51, 73, 156, 146, 132, 245],) ; & GUID
            }
        } impl winrt :: AsAbi for IColorHelperStatics
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for IColorHelperStatics
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } # [repr (C)] pub struct IColorHelperStatics2
        { ptr : * mut std :: ffi :: c_void } # [repr (C)] struct
        abi_IColorHelperStatics2
        {
            __0 : usize, __1 : usize, __2 : usize, __3 : usize, __4 : usize,
            __5 : usize, r#to_display_name : extern "system" fn
            (* const std :: ffi :: c_void, Color, & mut * mut std :: ffi ::
             c_void,) -> winrt :: ErrorCode,
        } impl IColorHelperStatics2
        {
            pub fn r#to_display_name (& self, color : & Color,) -> winrt ::
            Result < winrt :: String >
            {
                unsafe
                {
                    let mut __ok = std :: ptr :: null_mut () ;
                    ((*
                      (*
                       (self . ptr as * const * const
                        abi_IColorHelperStatics2))) . r#to_display_name)
                    (self . ptr, color, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            }
        } impl winrt :: TypeGuid for IColorHelperStatics2
        {
            fn type_guid () -> & 'static winrt :: Guid
            {
                static GUID : winrt :: Guid = winrt :: Guid :: from_values
                (618245890, 28336, 19348, &
                 [133, 92, 252, 240, 129, 141, 154, 22],) ; & GUID
            }
        } impl winrt :: AsAbi for IColorHelperStatics2
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for IColorHelperStatics2
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } # [repr (C)] pub struct IColors { ptr : * mut std :: ffi :: c_void }
        # [repr (C)] struct abi_IColors
        {
            __0 : usize, __1 : usize, __2 : usize, __3 : usize, __4 : usize,
            __5 : usize,
        } impl IColors { } impl winrt :: TypeGuid for IColors
        {
            fn type_guid () -> & 'static winrt :: Guid
            {
                static GUID : winrt :: Guid = winrt :: Guid :: from_values
                (2609681190, 19622, 19685, &
                 [137, 148, 158, 255, 101, 202, 189, 204],) ; & GUID
            }
        } impl winrt :: AsAbi for IColors
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for IColors
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } # [repr (C)] pub struct IColorsStatics
        { ptr : * mut std :: ffi :: c_void } # [repr (C)] struct
        abi_IColorsStatics
        {
            __0 : usize, __1 : usize, __2 : usize, __3 : usize, __4 : usize,
            __5 : usize, r#alice_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#antique_white : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#aqua : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#aquamarine : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#azure : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#beige : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#bisque : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#black : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#blanched_almond : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#blue_violet : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#brown : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#burly_wood : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#cadet_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#chartreuse : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#chocolate : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#coral : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#cornflower_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#cornsilk : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#crimson : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#cyan : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_cyan : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_goldenrod : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_gray : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_khaki : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_magenta : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_olive_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_orange : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_orchid : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_red : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_salmon : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_sea_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_slate_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_slate_gray : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_turquoise : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dark_violet : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#deep_pink : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#deep_sky_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dim_gray : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#dodger_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#firebrick : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#floral_white : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#forest_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#fuchsia : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#gainsboro : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#ghost_white : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#gold : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#goldenrod : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#gray : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#green_yellow : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#honeydew : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#hot_pink : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#indian_red : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#indigo : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#ivory : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#khaki : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#lavender : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#lavender_blush : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#lawn_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#lemon_chiffon : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_coral : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_cyan : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_goldenrod_yellow : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_gray : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_pink : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_salmon : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_sea_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_sky_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_slate_gray : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_steel_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#light_yellow : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#lime : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#lime_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#linen : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#magenta : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#maroon : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_aquamarine : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_orchid : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_purple : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_sea_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_slate_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_spring_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_turquoise : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#medium_violet_red : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#midnight_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#mint_cream : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#misty_rose : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#moccasin : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#navajo_white : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#navy : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#old_lace : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#olive : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#olive_drab : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#orange : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#orange_red : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#orchid : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#pale_goldenrod : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#pale_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#pale_turquoise : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#pale_violet_red : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#papaya_whip : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#peach_puff : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#peru : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#pink : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#plum : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#powder_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#purple : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#red : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#rosy_brown : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#royal_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#saddle_brown : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#salmon : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#sandy_brown : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#sea_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#sea_shell : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#sienna : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#silver : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#sky_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#slate_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#slate_gray : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#snow : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#spring_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#steel_blue : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#tan : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#teal : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#thistle : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#tomato : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#transparent : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#turquoise : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#violet : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#wheat : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#white : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#white_smoke : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#yellow : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode, r#yellow_green : extern "system" fn
            (* const std :: ffi :: c_void, & mut Color,) -> winrt ::
            ErrorCode,
        } impl IColorsStatics
        {
            pub fn r#alice_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#alice_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#antique_white (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#antique_white) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#aqua (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#aqua) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#aquamarine (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#aquamarine) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#azure (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#azure) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#beige (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#beige) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#bisque (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#bisque) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#black (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#black) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#blanched_almond (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#blanched_almond) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#blue_violet (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#blue_violet) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#brown (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#brown) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#burly_wood (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#burly_wood) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#cadet_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#cadet_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#chartreuse (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#chartreuse) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#chocolate (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#chocolate) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#coral (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#coral) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#cornflower_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#cornflower_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#cornsilk (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#cornsilk) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#crimson (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#crimson) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#cyan (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#cyan) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_cyan (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_cyan) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_goldenrod (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_goldenrod) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_gray (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_gray) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_khaki (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_khaki) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_magenta (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_magenta) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_olive_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_olive_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_orange (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_orange) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_orchid (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_orchid) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_red (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_red) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_salmon (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_salmon) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_sea_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_sea_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_slate_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_slate_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_slate_gray (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_slate_gray) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_turquoise (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_turquoise) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dark_violet (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dark_violet) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#deep_pink (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#deep_pink) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#deep_sky_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#deep_sky_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dim_gray (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dim_gray) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#dodger_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#dodger_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#firebrick (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#firebrick) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#floral_white (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#floral_white) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#forest_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#forest_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#fuchsia (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#fuchsia) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#gainsboro (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#gainsboro) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#ghost_white (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#ghost_white) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#gold (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#gold) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#goldenrod (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#goldenrod) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#gray (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#gray) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#green_yellow (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#green_yellow) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#honeydew (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#honeydew) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#hot_pink (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#hot_pink) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#indian_red (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#indian_red) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#indigo (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#indigo) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#ivory (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#ivory) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#khaki (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#khaki) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#lavender (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#lavender) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#lavender_blush (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#lavender_blush) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#lawn_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#lawn_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#lemon_chiffon (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#lemon_chiffon) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_coral (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_coral) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_cyan (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_cyan) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_goldenrod_yellow (& self,) -> winrt :: Result <
            Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_goldenrod_yellow) (self . ptr, & mut __ok,) .
                    ok_or (From :: from (__ok))
                }
            } pub fn r#light_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_gray (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_gray) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_pink (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_pink) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_salmon (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_salmon) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_sea_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_sea_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_sky_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_sky_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_slate_gray (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_slate_gray) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_steel_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_steel_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#light_yellow (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#light_yellow) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#lime (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#lime) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#lime_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#lime_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#linen (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#linen) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#magenta (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#magenta) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#maroon (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#maroon) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#medium_aquamarine (& self,) -> winrt :: Result < Color
            >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_aquamarine) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#medium_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#medium_orchid (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_orchid) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#medium_purple (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_purple) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#medium_sea_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_sea_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#medium_slate_blue (& self,) -> winrt :: Result < Color
            >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_slate_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#medium_spring_green (& self,) -> winrt :: Result <
            Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_spring_green) (self . ptr, & mut __ok,) .
                    ok_or (From :: from (__ok))
                }
            } pub fn r#medium_turquoise (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_turquoise) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#medium_violet_red (& self,) -> winrt :: Result < Color
            >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#medium_violet_red) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#midnight_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#midnight_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#mint_cream (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#mint_cream) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#misty_rose (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#misty_rose) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#moccasin (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#moccasin) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#navajo_white (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#navajo_white) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#navy (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#navy) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#old_lace (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#old_lace) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#olive (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#olive) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#olive_drab (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#olive_drab) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#orange (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#orange) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#orange_red (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#orange_red) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#orchid (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#orchid) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#pale_goldenrod (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#pale_goldenrod) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#pale_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#pale_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#pale_turquoise (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#pale_turquoise) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#pale_violet_red (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#pale_violet_red) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#papaya_whip (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#papaya_whip) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#peach_puff (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#peach_puff) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#peru (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#peru) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#pink (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#pink) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#plum (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#plum) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#powder_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#powder_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#purple (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#purple) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#red (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#red) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#rosy_brown (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#rosy_brown) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#royal_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#royal_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#saddle_brown (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#saddle_brown) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#salmon (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#salmon) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#sandy_brown (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#sandy_brown) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#sea_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#sea_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#sea_shell (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#sea_shell) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#sienna (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#sienna) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#silver (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#silver) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#sky_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#sky_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#slate_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#slate_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#slate_gray (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#slate_gray) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#snow (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#snow) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#spring_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#spring_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#steel_blue (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#steel_blue) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#tan (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#tan) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#teal (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#teal) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#thistle (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#thistle) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#tomato (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#tomato) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#transparent (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#transparent) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#turquoise (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#turquoise) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#violet (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#violet) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#wheat (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#wheat) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#white (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#white) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#white_smoke (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#white_smoke) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#yellow (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#yellow) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            } pub fn r#yellow_green (& self,) -> winrt :: Result < Color >
            {
                unsafe
                {
                    let mut __ok = Default :: default () ;
                    ((*
                      (* (self . ptr as * const * const abi_IColorsStatics)))
                     . r#yellow_green) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            }
        } impl winrt :: TypeGuid for IColorsStatics
        {
            fn type_guid () -> & 'static winrt :: Guid
            {
                static GUID : winrt :: Guid = winrt :: Guid :: from_values
                (3488951812, 52390, 17940, &
                 [161, 126, 117, 73, 16, 200, 74, 153],) ; & GUID
            }
        } impl winrt :: AsAbi for IColorsStatics
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for IColorsStatics
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } # [repr (C)] pub struct IUIContentRoot
        { ptr : * mut std :: ffi :: c_void } # [repr (C)] struct
        abi_IUIContentRoot
        {
            __0 : usize, __1 : usize, __2 : usize, __3 : usize, __4 : usize,
            __5 : usize, r#u_i_context : extern "system" fn
            (* const std :: ffi :: c_void, & mut * mut std :: ffi :: c_void,)
            -> winrt :: ErrorCode,
        } impl IUIContentRoot
        {
            pub fn r#u_i_context (& self,) -> winrt :: Result < UIContext >
            {
                unsafe
                {
                    let mut __ok = std :: ptr :: null_mut () ;
                    ((*
                      (* (self . ptr as * const * const abi_IUIContentRoot)))
                     . r#u_i_context) (self . ptr, & mut __ok,) . ok_or
                    (From :: from (__ok))
                }
            }
        } impl winrt :: TypeGuid for IUIContentRoot
        {
            fn type_guid () -> & 'static winrt :: Guid
            {
                static GUID : winrt :: Guid = winrt :: Guid :: from_values
                (503102150, 45931, 23737, &
                 [155, 197, 43, 122, 14, 221, 195, 120],) ; & GUID
            }
        } impl winrt :: AsAbi for IUIContentRoot
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for IUIContentRoot
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } # [repr (C)] pub struct IUIContext
        { ptr : * mut std :: ffi :: c_void } # [repr (C)] struct
        abi_IUIContext
        {
            __0 : usize, __1 : usize, __2 : usize, __3 : usize, __4 : usize,
            __5 : usize,
        } impl IUIContext { } impl winrt :: TypeGuid for IUIContext
        {
            fn type_guid () -> & 'static winrt :: Guid
            {
                static GUID : winrt :: Guid = winrt :: Guid :: from_values
                (3143432909, 23512, 22992, &
                 [165, 158, 28, 23, 164, 214, 210, 67],) ; & GUID
            }
        } impl winrt :: AsAbi for IUIContext
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for IUIContext
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } pub struct UIContentRoot { ptr : * mut std :: ffi :: c_void } impl
        UIContentRoot { } impl winrt :: TypeName for UIContentRoot
        { fn type_name () -> & 'static str { "Windows.UI.UIContentRoot" } }
        impl winrt :: AsAbi for UIContentRoot
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for UIContentRoot
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        } pub struct UIContext { ptr : * mut std :: ffi :: c_void } impl
        UIContext { } impl winrt :: TypeName for UIContext
        { fn type_name () -> & 'static str { "Windows.UI.UIContext" } } impl
        winrt :: AsAbi for UIContext
        {
            type In = * const std :: ffi :: c_void ; type Out = * mut * mut
            std :: ffi :: c_void ; fn as_abi_in (& self) -> Self :: In
            { self . ptr } fn as_abi_out (& mut self) -> Self :: Out
            { debug_assert ! (self . ptr . is_null ()) ; & mut self . ptr }
        } impl From < * mut std :: ffi :: c_void > for UIContext
        {
            fn from (ptr : * mut std :: ffi :: c_void) -> Self
            { Self { ptr } }
        }
    }
}
use windows::ui::*;

fn main() -> winrt::Result<()> {
    //test_reader();

    let color = Colors::red()?;
    println!("{:?}", color);
    assert!(color == ColorHelper::from_argb(255, 255, 0, 0)?);
    println!("woot!");

    Ok(())
}

fn test_reader() {
    let reader = winmd::Reader::from_files(&[
        r"C:\Windows\System32\WinMetadata\Windows.Foundation.winmd".to_string(),
    ])
    .unwrap();
    let t = reader
        .find_type("Windows.Foundation.IAsyncOperationWithProgress`2")
        .unwrap();
    let g = t.generics();

    if g.is_empty() {
        println!("{} is not generic", t.name());
    } else {
        println!("{} is generic", t.name());

        for param in g {
            print!("{}, ", param.name());
        }
    }
}

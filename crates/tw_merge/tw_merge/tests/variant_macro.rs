#[cfg(all(test, feature = "variant"))]
mod test {
    use tw_merge::*;

    #[derive(TwVariant, Debug, PartialEq)]
    enum BtnColor {
        #[tw(default, class = "bg-blue-500 text-blue-100")]
        Blue,
        #[tw(class = "bg-red-500 text-red-100")]
        Red,
    }

    #[test]
    fn btn_color() {
        assert_eq!(BtnColor::Blue.as_class(), "bg-blue-500 text-blue-100");
        assert_eq!(BtnColor::Red.as_class(), "bg-red-500 text-red-100");

        assert_eq!(BtnColor::default(), BtnColor::Blue);
    }

    #[test]
    fn btn_color_with_default() {
        #[derive(TwVariant)]
        #[tw(class = "text-white")]
        enum BtnColor {
            #[tw(default, class = "bg-blue-500")]
            Blue,
            #[tw(class = "bg-red-500")]
            Red,
        }
        assert_eq!(BtnColor::Blue.as_class(), "text-white bg-blue-500");
        assert_eq!(BtnColor::Red.as_class(), "text-white bg-red-500");
    }

    #[derive(TwVariant)]
    enum BtnSize {
        #[tw(default, class = "h-9 px-4 py-2")]
        Default,
        #[tw(class = "h-8 rounded-md px-3 text-xs")]
        Sm,
        #[tw(class = "h-10 rounded-lg px-8")]
        Lg,
    }

    #[derive(TwClass)]
    struct Btn {
        size: BtnSize,
        color: BtnColor,
    }

    #[test]
    fn test_btn() {
        let button = Btn { size: Default::default(), color: Default::default() };

        assert_eq!(button.to_class(), "h-9 px-4 py-2 bg-blue-500 text-blue-100");
        assert_eq!(button.with_class("text-lg"), "h-9 px-4 py-2 bg-blue-500 text-blue-100 text-lg", "append class");
        assert_eq!(button.with_class("text-red-500"), "h-9 px-4 py-2 bg-blue-500 text-red-500", "color conflict");
    }

    #[test]
    fn test_btn_no_merge() {
        #[derive(TwClass, Default)]
        #[tw(merger = TailwindJoin)]
        struct Btn {
            size: BtnSize,
            color: BtnColor,
        }

        let button = Btn::default();

        assert_eq!(button.to_class(), "h-9 px-4 py-2 bg-blue-500 text-blue-100");
        assert_eq!(button.with_class("h-10"), "h-9 px-4 py-2 bg-blue-500 text-blue-100 h-10")
    }

    #[test]
    fn test_class_builder() {
        assert_eq!(
            Btn::builder().size(BtnSize::Sm).color(BtnColor::Red).to_class(),
            "h-8 rounded-md px-3 text-xs bg-red-500 text-red-100"
        );

        assert_eq!(
            Btn::builder().size(BtnSize::Sm).color(BtnColor::Red).with_class("flex"),
            "h-8 rounded-md px-3 text-xs bg-red-500 text-red-100 flex"
        );

        assert_eq!(Btn::builder().size(BtnSize::Lg).to_class(), "h-10 rounded-lg px-8 bg-blue-500 text-blue-100");

        assert_eq!(Btn::builder().to_class(), "h-9 px-4 py-2 bg-blue-500 text-blue-100");

        assert_eq!(Btn::builder().with_class("grid"), "h-9 px-4 py-2 bg-blue-500 text-blue-100 grid");
    }

    #[test]
    fn variant_join() {
        assert_eq!(tw_merge!(BtnColor::Blue, "text-lg",), "bg-blue-500 text-blue-100 text-lg");
    }
}

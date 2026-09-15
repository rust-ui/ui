use ratatui::layout::{Constraint, Flex, Layout, Rect};

/// Helper function to create a centered rect using up certain percentage of the available rect
pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

/* ========================================================== */
/*                        🧪 TESTS 🧪                         */
/* ========================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popup_area_output_contained_within_input() {
        let area = Rect::new(0, 0, 100, 50);
        let result = popup_area(area, 80, 60);
        assert!(result.x >= area.x);
        assert!(result.y >= area.y);
        assert!(result.right() <= area.right());
        assert!(result.bottom() <= area.bottom());
    }

    #[test]
    fn popup_area_at_100_percent_equals_input() {
        let area = Rect::new(0, 0, 100, 50);
        let result = popup_area(area, 100, 100);
        assert_eq!(result.width, area.width);
        assert_eq!(result.height, area.height);
    }

    #[test]
    fn popup_area_reduces_dimensions() {
        let area = Rect::new(0, 0, 100, 50);
        let result = popup_area(area, 50, 50);
        assert!(result.width < area.width);
        assert!(result.height < area.height);
    }

    #[test]
    fn popup_area_is_centered() {
        let area = Rect::new(0, 0, 100, 50);
        let result = popup_area(area, 50, 50);
        let area_center_x = area.x + area.width / 2;
        let area_center_y = area.y + area.height / 2;
        let result_center_x = result.x + result.width / 2;
        let result_center_y = result.y + result.height / 2;
        assert!((result_center_x as i32 - area_center_x as i32).abs() <= 1);
        assert!((result_center_y as i32 - area_center_y as i32).abs() <= 1);
    }
}

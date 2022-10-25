use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::ListItem,
};

use crate::chat::{message::ChatMessage, user::User};

// We basically use ListItems in every UI, so a Trait that can make something to an ListItem is good
pub trait ListItemable {
    fn to_list_item(&self) -> ListItem;
}

impl ListItemable for User {
    fn to_list_item(&self) -> ListItem {
        let name_color = self.flair.to_color();
        let line = Spans::from(vec![Span::styled(
            format!("{}", &self.name),
            Style::default().fg(name_color),
        )]);
        ListItem::new(line)
    }
}

// Probably not possible, since Linewraps return multiple ListItems
// Would need to return a Vec<ListItem>
impl ListItemable for ChatMessage {
    fn to_list_item(&self) -> ListItem {
        todo!()
    }
}

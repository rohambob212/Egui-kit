import cargoToml from '../../rust/egui-kit/Cargo.toml?raw';
import libRs from '../../rust/egui-kit/src/lib.rs?raw';
import tokensRs from '../../rust/egui-kit/src/tokens.rs?raw';
import themeCoreRs from '../../rust/egui-kit/src/theme.rs?raw';
import componentsModRs from '../../rust/egui-kit/src/components/mod.rs?raw';
import rootRs from '../../rust/egui-kit/src/components/root.rs?raw';
import themeWidgetRs from '../../rust/egui-kit/src/components/theme.rs?raw';
import buttonRs from '../../rust/egui-kit/src/components/button.rs?raw';
import dropdownButtonRs from '../../rust/egui-kit/src/components/dropdown_button.rs?raw';
import iconRs from '../../rust/egui-kit/src/components/icon.rs?raw';
import badgeRs from '../../rust/egui-kit/src/components/badge.rs?raw';
import tagRs from '../../rust/egui-kit/src/components/tag.rs?raw';
import labelRs from '../../rust/egui-kit/src/components/label.rs?raw';
import kbdRs from '../../rust/egui-kit/src/components/kbd.rs?raw';
import avatarRs from '../../rust/egui-kit/src/components/avatar.rs?raw';
import checkboxRs from '../../rust/egui-kit/src/components/checkbox.rs?raw';
import radioRs from '../../rust/egui-kit/src/components/radio.rs?raw';
import switchRs from '../../rust/egui-kit/src/components/switch.rs?raw';
import toggleRs from '../../rust/egui-kit/src/components/toggle.rs?raw';
import sliderRs from '../../rust/egui-kit/src/components/slider.rs?raw';
import ratingRs from '../../rust/egui-kit/src/components/rating.rs?raw';
import progressRs from '../../rust/egui-kit/src/components/progress.rs?raw';
import spinnerRs from '../../rust/egui-kit/src/components/spinner.rs?raw';
import skeletonRs from '../../rust/egui-kit/src/components/skeleton.rs?raw';
import shimmerRs from '../../rust/egui-kit/src/components/shimmer.rs?raw';
import inputRs from '../../rust/egui-kit/src/components/input.rs?raw';
import inputGroupRs from '../../rust/egui-kit/src/components/input_group.rs?raw';
import textareaRs from '../../rust/egui-kit/src/components/textarea.rs?raw';
import numberInputRs from '../../rust/egui-kit/src/components/number_input.rs?raw';
import otpInputRs from '../../rust/egui-kit/src/components/otp_input.rs?raw';
import selectRs from '../../rust/egui-kit/src/components/select.rs?raw';
import comboboxRs from '../../rust/egui-kit/src/components/combobox.rs?raw';
import formRs from '../../rust/egui-kit/src/components/form.rs?raw';
import questionnaireRs from '../../rust/egui-kit/src/components/questionnaire.rs?raw';
import clipboardRs from '../../rust/egui-kit/src/components/clipboard.rs?raw';
import alertRs from '../../rust/egui-kit/src/components/alert.rs?raw';
import emptyRs from '../../rust/egui-kit/src/components/empty.rs?raw';
import attachmentRs from '../../rust/egui-kit/src/components/attachment.rs?raw';
import imageRs from '../../rust/egui-kit/src/components/image.rs?raw';
import calendarRs from '../../rust/egui-kit/src/components/calendar.rs?raw';
import datePickerRs from '../../rust/egui-kit/src/components/date_picker.rs?raw';
import timeFieldRs from '../../rust/egui-kit/src/components/time_field.rs?raw';
import colorPickerRs from '../../rust/egui-kit/src/components/color_picker.rs?raw';
import markerRs from '../../rust/egui-kit/src/components/marker.rs?raw';
import speechRs from '../../rust/egui-kit/src/components/speech.rs?raw';
import groupBoxRs from '../../rust/egui-kit/src/components/group_box.rs?raw';
import descriptionListRs from '../../rust/egui-kit/src/components/description_list.rs?raw';
import gridLayoutRs from '../../rust/egui-kit/src/components/grid_layout.rs?raw';
import scrollableRs from '../../rust/egui-kit/src/components/scrollable.rs?raw';
import resizableRs from '../../rust/egui-kit/src/components/resizable.rs?raw';
import accordionRs from '../../rust/egui-kit/src/components/accordion.rs?raw';
import collapsibleRs from '../../rust/egui-kit/src/components/collapsible.rs?raw';
import tabsRs from '../../rust/egui-kit/src/components/tabs.rs?raw';
import stepperRs from '../../rust/egui-kit/src/components/stepper.rs?raw';
import paginationRs from '../../rust/egui-kit/src/components/pagination.rs?raw';
import dialogRs from '../../rust/egui-kit/src/components/dialog.rs?raw';
import alertDialogRs from '../../rust/egui-kit/src/components/alert_dialog.rs?raw';
import sheetRs from '../../rust/egui-kit/src/components/sheet.rs?raw';
import popoverRs from '../../rust/egui-kit/src/components/popover.rs?raw';
import tooltipRs from '../../rust/egui-kit/src/components/tooltip.rs?raw';
import hoverCardRs from '../../rust/egui-kit/src/components/hover_card.rs?raw';
import menuRs from '../../rust/egui-kit/src/components/menu.rs?raw';
import notificationRs from '../../rust/egui-kit/src/components/notification.rs?raw';
import focusTrapRs from '../../rust/egui-kit/src/components/focus_trap.rs?raw';
import commandRs from '../../rust/egui-kit/src/components/command.rs?raw';
import part1ShowcaseRs from '../../rust/egui-kit/examples/part1_showcase.rs?raw';
import part2ShowcaseRs from '../../rust/egui-kit/examples/part2_showcase.rs?raw';
import part3ShowcaseRs from '../../rust/egui-kit/examples/part3_showcase.rs?raw';
import part4ShowcaseRs from '../../rust/egui-kit/examples/part4_showcase.rs?raw';
import part5ShowcaseRs from '../../rust/egui-kit/examples/part5_showcase.rs?raw';
import part6ShowcaseRs from '../../rust/egui-kit/examples/part6_showcase.rs?raw';
import type { CrateFileEntry } from '../types/kit';

export const CRATE_FILES: CrateFileEntry[] = [
  { path: 'Cargo.toml', title: 'Cargo.toml', group: 'Workspace', content: cargoToml },
  { path: 'src/lib.rs', title: 'src/lib.rs', group: 'Core', content: libRs },
  { path: 'src/tokens.rs', title: 'src/tokens.rs', group: 'Core', content: tokensRs },
  { path: 'src/theme.rs', title: 'src/theme.rs', group: 'Core', content: themeCoreRs },
  { path: 'src/components/mod.rs', title: 'src/components/mod.rs', group: 'Components', content: componentsModRs },
  { path: 'src/components/root.rs', title: 'root.rs', group: 'Components', content: rootRs },
  { path: 'src/components/theme.rs', title: 'theme.rs', group: 'Components', content: themeWidgetRs },
  { path: 'src/components/button.rs', title: 'button.rs', group: 'Components', content: buttonRs },
  { path: 'src/components/dropdown_button.rs', title: 'dropdown_button.rs', group: 'Components', content: dropdownButtonRs },
  { path: 'src/components/icon.rs', title: 'icon.rs', group: 'Components', content: iconRs },
  { path: 'src/components/badge.rs', title: 'badge.rs', group: 'Components', content: badgeRs },
  { path: 'src/components/tag.rs', title: 'tag.rs', group: 'Components', content: tagRs },
  { path: 'src/components/label.rs', title: 'label.rs', group: 'Components', content: labelRs },
  { path: 'src/components/kbd.rs', title: 'kbd.rs', group: 'Components', content: kbdRs },
  { path: 'src/components/avatar.rs', title: 'avatar.rs', group: 'Components', content: avatarRs },
  { path: 'src/components/checkbox.rs', title: 'checkbox.rs', group: 'Components', content: checkboxRs },
  { path: 'src/components/radio.rs', title: 'radio.rs', group: 'Components', content: radioRs },
  { path: 'src/components/switch.rs', title: 'switch.rs', group: 'Components', content: switchRs },
  { path: 'src/components/toggle.rs', title: 'toggle.rs', group: 'Components', content: toggleRs },
  { path: 'src/components/slider.rs', title: 'slider.rs', group: 'Components', content: sliderRs },
  { path: 'src/components/rating.rs', title: 'rating.rs', group: 'Components', content: ratingRs },
  { path: 'src/components/progress.rs', title: 'progress.rs', group: 'Components', content: progressRs },
  { path: 'src/components/spinner.rs', title: 'spinner.rs', group: 'Components', content: spinnerRs },
  { path: 'src/components/skeleton.rs', title: 'skeleton.rs', group: 'Components', content: skeletonRs },
  { path: 'src/components/shimmer.rs', title: 'shimmer.rs', group: 'Components', content: shimmerRs },
  { path: 'src/components/input.rs', title: 'input.rs', group: 'Components', content: inputRs },
  { path: 'src/components/input_group.rs', title: 'input_group.rs', group: 'Components', content: inputGroupRs },
  { path: 'src/components/textarea.rs', title: 'textarea.rs', group: 'Components', content: textareaRs },
  { path: 'src/components/number_input.rs', title: 'number_input.rs', group: 'Components', content: numberInputRs },
  { path: 'src/components/otp_input.rs', title: 'otp_input.rs', group: 'Components', content: otpInputRs },
  { path: 'src/components/select.rs', title: 'select.rs', group: 'Components', content: selectRs },
  { path: 'src/components/combobox.rs', title: 'combobox.rs', group: 'Components', content: comboboxRs },
  { path: 'src/components/form.rs', title: 'form.rs', group: 'Components', content: formRs },
  { path: 'src/components/questionnaire.rs', title: 'questionnaire.rs', group: 'Components', content: questionnaireRs },
  { path: 'src/components/clipboard.rs', title: 'clipboard.rs', group: 'Components', content: clipboardRs },
  { path: 'src/components/alert.rs', title: 'alert.rs', group: 'Components', content: alertRs },
  { path: 'src/components/empty.rs', title: 'empty.rs', group: 'Components', content: emptyRs },
  { path: 'src/components/attachment.rs', title: 'attachment.rs', group: 'Components', content: attachmentRs },
  { path: 'src/components/image.rs', title: 'image.rs', group: 'Components', content: imageRs },
  { path: 'src/components/calendar.rs', title: 'calendar.rs', group: 'Components', content: calendarRs },
  { path: 'src/components/date_picker.rs', title: 'date_picker.rs', group: 'Components', content: datePickerRs },
  { path: 'src/components/time_field.rs', title: 'time_field.rs', group: 'Components', content: timeFieldRs },
  { path: 'src/components/color_picker.rs', title: 'color_picker.rs', group: 'Components', content: colorPickerRs },
  { path: 'src/components/marker.rs', title: 'marker.rs', group: 'Components', content: markerRs },
  { path: 'src/components/speech.rs', title: 'speech.rs', group: 'Components', content: speechRs },
  { path: 'src/components/group_box.rs', title: 'group_box.rs', group: 'Components', content: groupBoxRs },
  { path: 'src/components/description_list.rs', title: 'description_list.rs', group: 'Components', content: descriptionListRs },
  { path: 'src/components/grid_layout.rs', title: 'grid_layout.rs', group: 'Components', content: gridLayoutRs },
  { path: 'src/components/scrollable.rs', title: 'scrollable.rs', group: 'Components', content: scrollableRs },
  { path: 'src/components/resizable.rs', title: 'resizable.rs', group: 'Components', content: resizableRs },
  { path: 'src/components/accordion.rs', title: 'accordion.rs', group: 'Components', content: accordionRs },
  { path: 'src/components/collapsible.rs', title: 'collapsible.rs', group: 'Components', content: collapsibleRs },
  { path: 'src/components/tabs.rs', title: 'tabs.rs', group: 'Components', content: tabsRs },
  { path: 'src/components/stepper.rs', title: 'stepper.rs', group: 'Components', content: stepperRs },
  { path: 'src/components/pagination.rs', title: 'pagination.rs', group: 'Components', content: paginationRs },
  { path: 'src/components/dialog.rs', title: 'dialog.rs', group: 'Components', content: dialogRs },
  { path: 'src/components/alert_dialog.rs', title: 'alert_dialog.rs', group: 'Components', content: alertDialogRs },
  { path: 'src/components/sheet.rs', title: 'sheet.rs', group: 'Components', content: sheetRs },
  { path: 'src/components/popover.rs', title: 'popover.rs', group: 'Components', content: popoverRs },
  { path: 'src/components/tooltip.rs', title: 'tooltip.rs', group: 'Components', content: tooltipRs },
  { path: 'src/components/hover_card.rs', title: 'hover_card.rs', group: 'Components', content: hoverCardRs },
  { path: 'src/components/menu.rs', title: 'menu.rs', group: 'Components', content: menuRs },
  { path: 'src/components/notification.rs', title: 'notification.rs', group: 'Components', content: notificationRs },
  { path: 'src/components/focus_trap.rs', title: 'focus_trap.rs', group: 'Components', content: focusTrapRs },
  { path: 'src/components/command.rs', title: 'command.rs', group: 'Components', content: commandRs },
  { path: 'examples/part1_showcase.rs', title: 'part1_showcase.rs', group: 'Examples', content: part1ShowcaseRs },
  { path: 'examples/part2_showcase.rs', title: 'part2_showcase.rs', group: 'Examples', content: part2ShowcaseRs },
  { path: 'examples/part3_showcase.rs', title: 'part3_showcase.rs', group: 'Examples', content: part3ShowcaseRs },
  { path: 'examples/part4_showcase.rs', title: 'part4_showcase.rs', group: 'Examples', content: part4ShowcaseRs },
  { path: 'examples/part5_showcase.rs', title: 'part5_showcase.rs', group: 'Examples', content: part5ShowcaseRs },
  { path: 'examples/part6_showcase.rs', title: 'part6_showcase.rs', group: 'Examples', content: part6ShowcaseRs },
];

export function getCrateFileByPath(path: string): CrateFileEntry | undefined {
  return CRATE_FILES.find((f) => f.path === path);
}

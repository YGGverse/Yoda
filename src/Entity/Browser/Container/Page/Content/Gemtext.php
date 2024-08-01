<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \Exception;
use \Gdk;
use \GdkEvent;
use \Gtk;
use \GtkLabel;
use \Pango;

use \Yggverse\Yoda\Model\Gtk\Pango\Markup\Gemtext as Markup;

# use \Yggverse\Gemtext\Parser\Link as LinkParser;

class Gemtext extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content\Markup
{
    public function set(
        string $source
    ): void
    {
        Gtk::timeout_add( // await for renderer dimensions init
            1, function(?string $title = null) use ($source)
            {
                if ($this->content->page->content->gtk->get_allocated_width() > Markup::WRAP_WIDTH)
                {
                    $this->gtk->set_markup(
                        Markup::format(
                            $source,
                            $this->content->page->navbar->request->getValue(),
                            $this->content->page->content->gtk->get_allocated_width(),
                            $title
                        )
                    );

                    // Update title by gemtext H1 tag (on available)
                    if ($title)
                    {
                        // Set new title
                        $this->content->page->title->setValue(
                            $title
                        );

                        // Update tooltip
                        $this->content->page->title->setTooltip(
                            $title
                        );

                        // Refresh header by new title if current page is active
                        if ($this->content->page === $this->content->page->container->tab->get())
                        {
                            $this->content->page->container->browser->header->setTitle(
                                $this->content->page->title->getValue(),
                                $this->content->page->title->getSubtitle()
                            );
                        }
                    };

                    return false; // stop
                }
            }
        );
    }

    protected function _onActivateLink(
        GtkLabel $label,
        string $href
    ): bool
    {
        // Update request entry
        $this->content->page->navbar->request->setValue(
            $href
        );

        // Update page
        $this->content->page->update();

        // Prevent propagation for supported protocols
        return in_array(
            parse_url(
                $href,
                PHP_URL_SCHEME
            ),
            [
                'nex',
                'gemini',
                'file'
            ]
        );
    }

    protected function _onButtonPress(
        GtkLabel $label,
        GdkEvent $event
    ): bool
    {
        // Open link in the new tab on middle button click
        if ($event->button->button == Gdk::BUTTON_MIDDLE)
        {
            // Detect cursor position
            $result = $label->get_layout()->xy_to_index(
                $event->button->x * Pango::SCALE,
                $event->button->y * Pango::SCALE
            );

            // Position detected
            if ($result)
            {
                // Get entire line from source

                /* @TODO incorrect offset index_
                if ($line = $this->_line($result['index_']))
                {
                    // Parse gemtext href
                    if ($href = LinkParser::getAddress($line))
                    {
                        // Open
                        $this->content->page->container->tab->append(
                            $href,
                            true,
                            false
                        );

                        return true;
                    }
                } */
            }
        }

        return false;
    }
}